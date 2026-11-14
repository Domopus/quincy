package session

import (
	"testing"
	"time"

	"go.mau.fi/whatsmeow/proto/waE2E"
	"go.mau.fi/whatsmeow/types"
	"go.mau.fi/whatsmeow/types/events"
	"google.golang.org/protobuf/proto"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/quincyapi"
)

func message(chat types.JID, fromMe bool, msg *waE2E.Message) *events.Message {
	return &events.Message{
		Info: types.MessageInfo{
			MessageSource: types.MessageSource{
				Chat:     chat,
				Sender:   types.NewJID("33600000000", types.DefaultUserServer),
				IsFromMe: fromMe,
				IsGroup:  chat.Server == types.GroupServer,
			},
			ID:        "MSG1",
			Timestamp: time.Date(2026, 11, 14, 10, 0, 0, 0, time.UTC),
		},
		Message: msg,
	}
}

var (
	direct = types.NewJID("33611111111", types.DefaultUserServer)
	group  = types.NewJID("120363000000000000", types.GroupServer)
)

func TestSkipsOwnMessagesStatusAndChannels(t *testing.T) {
	text := &waE2E.Message{Conversation: proto.String("hi")}
	for name, evt := range map[string]*events.Message{
		"own":     message(direct, true, text),
		"status":  message(types.StatusBroadcastJID, false, text),
		"channel": message(types.NewJID("123", types.NewsletterServer), false, text),
	} {
		if _, ok := toIncoming(evt, quincyapi.Live, "c"); ok {
			t.Errorf("%s message was forwarded", name)
		}
	}
}

func TestGroupMessageWithPreviewAndMusicMetadata(t *testing.T) {
	evt := message(group, false, &waE2E.Message{ExtendedTextMessage: &waE2E.ExtendedTextMessage{
		Text:          proto.String("écoute https://open.spotify.com/track/abc"),
		MatchedText:   proto.String("https://open.spotify.com/track/abc"),
		Title:         proto.String("Come Together - Remastered 2009"),
		Description:   proto.String("The Beatles · Abbey Road · Song · 1969"),
		JPEGThumbnail: make([]byte, 1234),
		MusicMetadata: &waE2E.EmbeddedMusic{
			Author:          proto.String("The Beatles"),
			Title:           proto.String("Come Together"),
			IsExplicit:      proto.Bool(false),
			ArtworkMediaKey: []byte("secret-key"),
		},
	}})

	got, ok := toIncoming(evt, quincyapi.CatchUp, "conn-1")
	if !ok {
		t.Fatal("message was skipped")
	}
	if got.Conversation.Kind != quincyapi.Group || got.Origin != quincyapi.CatchUp || got.ConnectionId != "conn-1" {
		t.Errorf("wrong envelope: %+v", got)
	}
	if got.Text == nil || *got.Text != "écoute https://open.spotify.com/track/abc" {
		t.Errorf("text = %v", got.Text)
	}
	p := got.LinkPreview
	if p == nil || *p.Url != "https://open.spotify.com/track/abc" || *p.ThumbnailBytes != 1234 {
		t.Fatalf("preview = %+v", p)
	}
	music := *p.MusicMetadata
	if music["author"] != "The Beatles" || music["is_explicit"] != false {
		t.Errorf("music = %v", music)
	}
	if _, leaked := music["artwork_media_key"]; leaked {
		t.Error("artwork key was forwarded")
	}
}

func TestTextFromCaptionAndNoPreview(t *testing.T) {
	evt := message(direct, false, &waE2E.Message{ImageMessage: &waE2E.ImageMessage{
		Caption: proto.String("https://www.deezer.com/track/1"),
	}})
	got, _ := toIncoming(evt, quincyapi.Live, "c")
	if got.Text == nil || *got.Text != "https://www.deezer.com/track/1" || got.LinkPreview != nil {
		t.Errorf("got text %v, preview %v", got.Text, got.LinkPreview)
	}
	if got.Conversation.Kind != quincyapi.Direct {
		t.Errorf("kind = %s", got.Conversation.Kind)
	}
}
