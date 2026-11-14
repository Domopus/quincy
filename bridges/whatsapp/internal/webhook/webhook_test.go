package webhook

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/quincyapi"
)

func TestOnlyNoContentCountsAsStored(t *testing.T) {
	for status, wantErr := range map[int]bool{
		http.StatusNoContent:           false,
		http.StatusUnauthorized:        true,
		http.StatusInternalServerError: true,
	} {
		var gotAuth string
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			gotAuth = r.Header.Get("Authorization")
			w.WriteHeader(status)
		}))

		client, err := New(server.URL, "s3cret")
		if err != nil {
			t.Fatal(err)
		}
		err = client.Send(context.Background(), quincyapi.IncomingMessage{
			Source: "whatsapp", MessageId: "1", SentAt: time.Now(), Origin: quincyapi.Live,
		})
		server.Close()

		if (err != nil) != wantErr {
			t.Errorf("status %d: err = %v", status, err)
		}
		if gotAuth != "Bearer s3cret" {
			t.Errorf("authorization header = %q", gotAuth)
		}
	}
}
