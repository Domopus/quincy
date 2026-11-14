// Package session runs one whatsmeow linked-device session and hands every
// message to a Sender (ADR-0002).
package session

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"log/slog"
	"sync/atomic"
	"time"

	"go.mau.fi/whatsmeow"
	"go.mau.fi/whatsmeow/store/sqlstore"
	"go.mau.fi/whatsmeow/types"
	"go.mau.fi/whatsmeow/types/events"
	waLog "go.mau.fi/whatsmeow/util/log"
	_ "modernc.org/sqlite"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/quincyapi"
)

// Sender delivers one message to the server; nil means it was stored.
type Sender interface {
	Send(ctx context.Context, msg quincyapi.IncomingMessage) error
}

type Config struct {
	// StorePath is the SQLite file holding the device keys, the only thing the bridge stores.
	StorePath    string
	ConnectionID string
	// PairPhone, when the store has no device yet, is the number to pair with (international format, digits only).
	PairPhone string
}

// ErrLoggedOut means WhatsApp ended the link; the User must pair again.
var ErrLoggedOut = errors.New("logged out by WhatsApp")

// Run connects and forwards messages until ctx ends or WhatsApp logs the device out.
func Run(ctx context.Context, cfg Config, sender Sender) error {
	db, err := sql.Open("sqlite", "file:"+cfg.StorePath+"?_pragma=foreign_keys(1)&_pragma=journal_mode(WAL)")
	if err != nil {
		return err
	}
	defer db.Close()
	container := sqlstore.NewWithDB(db, "sqlite3", waLog.Noop)
	if err := container.Upgrade(ctx); err != nil {
		return fmt.Errorf("device store: %w", err)
	}
	device, err := container.GetFirstDevice(ctx)
	if err != nil {
		return err
	}

	client := whatsmeow.NewClient(device, waLog.Noop)
	h := &handler{ctx: ctx, client: client, sender: sender, connectionID: cfg.ConnectionID, loggedOut: make(chan struct{})}
	client.AddEventHandlerWithSuccessStatus(h.handle)

	if client.Store.ID == nil {
		if err := pair(ctx, client, cfg.PairPhone); err != nil {
			return err
		}
	} else if err := client.Connect(); err != nil {
		return err
	}
	defer client.Disconnect()

	select {
	case <-ctx.Done():
		return nil
	case <-h.loggedOut:
		return ErrLoggedOut
	}
}

// pair links this bridge to the phone with an 8-character code.
func pair(ctx context.Context, client *whatsmeow.Client, phone string) error {
	if phone == "" {
		return errors.New("no paired device yet: pass the phone number to pair with")
	}
	qr, err := client.GetQRChannel(ctx)
	if err != nil {
		return err
	}
	if err := client.Connect(); err != nil {
		return err
	}
	<-qr // the first QR event means the connection is ready for code pairing
	code, err := client.PairPhone(ctx, phone, true, whatsmeow.PairClientChrome, "Chrome (Mac OS)")
	if err != nil {
		return fmt.Errorf("pairing code: %w", err)
	}
	fmt.Printf("\nOn the phone: WhatsApp > Linked devices > Link with phone number, then type: %s\n\n", code)
	go func() {
		for range qr { // drain; pairing ends with PairSuccess or a timeout
		}
	}()
	return nil
}

type handler struct {
	ctx          context.Context
	client       *whatsmeow.Client
	sender       Sender
	connectionID string
	catchingUp   atomic.Bool
	loggedOut    chan struct{}
	once         atomic.Bool
}

// handle returns false only when a message could not be stored, so
// whatsmeow withholds the receipt (ADR-0002).
func (h *handler) handle(evt any) bool {
	switch e := evt.(type) {
	case *events.Message:
		origin := quincyapi.Live
		if h.catchingUp.Load() {
			origin = quincyapi.CatchUp
		}
		return h.forward(e, origin)
	case *events.HistorySync:
		h.forwardHistory(e)
	case *events.OfflineSyncPreview:
		h.catchingUp.Store(true)
	case *events.OfflineSyncCompleted:
		h.catchingUp.Store(false)
		slog.Info("caught up", "missed_events", e.Count)
	case *events.PairSuccess:
		slog.Info("paired", "device", e.ID.String())
	case *events.Connected:
		slog.Info("connected")
	case *events.Disconnected:
		slog.Warn("disconnected")
	case *events.LoggedOut:
		slog.Error("logged out by WhatsApp", "reason", e.Reason.String())
		if h.once.CompareAndSwap(false, true) {
			close(h.loggedOut)
		}
	}
	return true
}

func (h *handler) forward(evt *events.Message, origin quincyapi.Origin) bool {
	msg, ok := toIncoming(evt, origin, h.connectionID)
	if !ok {
		return true
	}
	ctx, cancel := context.WithTimeout(h.ctx, 15*time.Second)
	defer cancel()
	if err := h.sender.Send(ctx, msg); err != nil {
		// Never log the message itself: only its ID.
		slog.Error("message not delivered to server", "id", msg.MessageId, "err", err)
		return false
	}
	return true
}

// forwardHistory sends the messages of a history sync blob. They are not
// resent by WhatsApp, so a failure is only logged.
func (h *handler) forwardHistory(evt *events.HistorySync) {
	sent, failed := 0, 0
	for _, conv := range evt.Data.GetConversations() {
		chat, err := types.ParseJID(conv.GetID())
		if err != nil {
			continue
		}
		for _, item := range conv.GetMessages() {
			msg, err := h.client.ParseWebMessage(chat, item.GetMessage())
			if err != nil {
				continue
			}
			if h.forward(msg, quincyapi.HistorySync) {
				sent++
			} else {
				failed++
			}
		}
	}
	slog.Info("history sync", "type", evt.Data.GetSyncType().String(), "messages", sent, "failed", failed)
}
