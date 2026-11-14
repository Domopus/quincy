// Command quincy-whatsapp is the WhatsApp bridge: one linked-device session
// whose messages all go to the Quincy server's webhook (ADR-0002).
package main

import (
	"context"
	"errors"
	"flag"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"syscall"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/session"
	"github.com/Domopus/quincy/bridges/whatsapp/internal/webhook"
)

func main() {
	phone := flag.String("pair", "", "phone number to pair with, digits only (first run)")
	flag.Parse()

	secret := os.Getenv("BRIDGE_SECRET")
	if secret == "" {
		fail("BRIDGE_SECRET is not set")
	}
	storePath, err := defaultStorePath()
	if err != nil {
		fail(err.Error())
	}

	sender, err := webhook.New(env("QUINCY_SERVER_URL", "http://127.0.0.1:8080"), secret)
	if err != nil {
		fail(err.Error())
	}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	err = session.Run(ctx, session.Config{
		StorePath:    env("WHATSAPP_STORE", storePath),
		ConnectionID: env("CONNECTION_ID", "whatsapp-dev"),
		PairPhone:    *phone,
	}, sender)
	if errors.Is(err, session.ErrLoggedOut) {
		fail("WhatsApp ended the link; delete the store and pair again")
	}
	if err != nil {
		fail(err.Error())
	}
}

// defaultStorePath keeps the device keys outside the repo, readable by the owner only.
func defaultStorePath() (string, error) {
	dir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}
	dir = filepath.Join(dir, "Quincy")
	if err := os.MkdirAll(dir, 0o700); err != nil {
		return "", err
	}
	return filepath.Join(dir, "whatsapp.db"), nil
}

func env(key, fallback string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return fallback
}

func fail(msg string) {
	slog.Error(msg)
	os.Exit(1)
}
