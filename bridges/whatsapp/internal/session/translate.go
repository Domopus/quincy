package session

import (
	"go.mau.fi/whatsmeow/proto/waE2E"
	"go.mau.fi/whatsmeow/types"
	"go.mau.fi/whatsmeow/types/events"

	"github.com/Domopus/quincy/bridges/whatsapp/internal/quincyapi"
)

const source = "whatsapp"

// toIncoming turns a WhatsApp message into the shared webhook format.
// It returns false for messages Quincy never needs: the User's own,
// status updates and channels.
func toIncoming(evt *events.Message, origin quincyapi.Origin, connectionID string) (quincyapi.IncomingMessage, bool) {
	info := evt.Info
	if info.IsFromMe || info.Chat == types.StatusBroadcastJID || info.Chat.Server == types.NewsletterServer {
		return quincyapi.IncomingMessage{}, false
	}

	kind := quincyapi.Direct
	if info.IsGroup {
		kind = quincyapi.Group
	}

	return quincyapi.IncomingMessage{
		Source:       source,
		MessageId:    info.ID,
		ConnectionId: connectionID,
		Sender:       info.Sender.ToNonAD().String(),
		Conversation: quincyapi.Conversation{Id: info.Chat.String(), Kind: kind},
		Text:         text(evt.Message),
		LinkPreview:  preview(evt.Message.GetExtendedTextMessage()),
		SentAt:       info.Timestamp.UTC(),
		Origin:       origin,
	}, true
}

// text is what the sender wrote, wherever WhatsApp put it.
func text(msg *waE2E.Message) *string {
	for _, candidate := range []string{
		msg.GetConversation(),
		msg.GetExtendedTextMessage().GetText(),
		msg.GetImageMessage().GetCaption(),
		msg.GetVideoMessage().GetCaption(),
	} {
		if candidate != "" {
			return &candidate
		}
	}
	return nil
}

// preview keeps the link preview's descriptive fields; the thumbnail is
// reduced to its size.
func preview(ext *waE2E.ExtendedTextMessage) *quincyapi.LinkPreview {
	if ext.GetMatchedText() == "" && ext.GetTitle() == "" {
		return nil
	}
	p := &quincyapi.LinkPreview{
		Url:         nonEmpty(ext.GetMatchedText()),
		Title:       nonEmpty(ext.GetTitle()),
		Description: nonEmpty(ext.GetDescription()),
	}
	if n := len(ext.GetJPEGThumbnail()); n > 0 {
		size := int32(n)
		p.ThumbnailBytes = &size
	}
	if music := ext.GetMusicMetadata(); music != nil {
		// Only the fields that describe the song; artwork keys stay out.
		fields := map[string]any{
			"song_id":            music.GetSongID(),
			"author":             music.GetAuthor(),
			"title":              music.GetTitle(),
			"artist_attribution": music.GetArtistAttribution(),
		}
		if music.IsExplicit != nil {
			fields["is_explicit"] = music.GetIsExplicit()
		}
		p.MusicMetadata = &fields
	}
	return p
}

func nonEmpty(s string) *string {
	if s == "" {
		return nil
	}
	return &s
}
