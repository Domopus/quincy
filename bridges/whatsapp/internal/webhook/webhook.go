// Package webhook posts every message to the Quincy server (ADR-0002, ADR-0009).
package webhook

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/quincyapi"
)

// Client sends messages to the server's `POST /internal/messages`.
type Client struct {
	api *quincyapi.Client
}

func New(serverURL, bridgeSecret string) (*Client, error) {
	api, err := quincyapi.NewClient(
		serverURL,
		quincyapi.WithHTTPClient(&http.Client{Timeout: 10 * time.Second}),
		quincyapi.WithRequestEditorFn(func(_ context.Context, req *http.Request) error {
			req.Header.Set("Authorization", "Bearer "+bridgeSecret)
			return nil
		}),
	)
	if err != nil {
		return nil, err
	}
	return &Client{api: api}, nil
}

// Send returns nil only when the server stored the message, so the caller
// may confirm receipt to the chat platform.
func (c *Client) Send(ctx context.Context, msg quincyapi.IncomingMessage) error {
	resp, err := c.api.Receive(ctx, msg)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	_, _ = io.Copy(io.Discard, resp.Body)
	if resp.StatusCode != http.StatusNoContent {
		return fmt.Errorf("server answered %s", resp.Status)
	}
	return nil
}
