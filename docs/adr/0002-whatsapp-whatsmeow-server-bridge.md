# 0002 — WhatsApp via whatsmeow in a server-side Go bridge

Status: Accepted (2026-10-18), amended same day: filtering moved to Rust; delivery by webhook.

## Context

Quincy must see links sent in all of the User's WhatsApp conversations, including muted ones. There is no official API for reading a personal account.

## Decision

A thin Go bridge on the server runs one whatsmeow linked-device session per User. It knows nothing about Quincy: no Shares, no Users, no matching.

- **Storage:** only whatsmeow's own session store (device keys), so restarts never force a re-pair, plus the encrypted buffer of messages Rust hasn't accepted yet (see Consequences). No Quincy data.
- **Incoming messages → webhook:** for every message, the bridge POSTs (source, user, sender, conversation, text, link preview, time; shared format per ADR-0009) to the Rust server's `POST /internal/messages`, then forgets it. Rust filters for music URLs and drops everything else (ADR-0003). Connection lost/restored is sent the same way.
- **Commands → bridge API:** Rust calls the bridge to start pairing (returns the 8-char code), unlink, and get status. The bridge also reports a logout by WhatsApp to Rust, which prompts the User to reconnect (added 2026-10-25).
- **Contracts (ADR-0001):** Rust owns the webhook endpoint (`utoipa` → generated Go client, `oapi-codegen`); Go owns the commands API (`huma` → generated Rust client, `progenitor`). The bridge is configured with Rust's URL and a shared secret.

## Rejected

- **Android notification listener (on device)** — private and no ban risk, but misses muted chats and doesn't exist on iOS.
- **whatsapp-rust** — very active but pre-1.0 (v0.7, Aug 2026). Revisit if it matures; it would remove Go.
- **Baileys (Node)** — independent reimplementation, more breakage reports, heavier per connection.
- **mautrix-whatsapp** — needs a Matrix homeserver.
- **Filtering in the bridge** — would put logic in Go; Rust holds all logic (ADR-0003), and the Go part stays easy to replace.
- **Rust polling a GET** — the bridge would have to keep private messages waiting until fetched.
- **SSE stream from the bridge** — works (Go `huma` documents it), but SSE is idiomatic for server → browser, not service → service; Rust's `progenitor` has no SSE client, so the reader would be hand-written. Webhooks are the conventional pattern (also used by whatsmeow REST wrappers like wuzapi) and fully generated.

## Consequences

- Unofficial client: WhatsApp can ban the User's number. Quincy only reads, which is the low-risk end; never send messages.
- Decrypted messages pass from bridge to Rust in memory — keep both on the same machine or private network; never log or store message text beyond the music links and the message written with them. The only exception is the bridge's encrypted buffer: never readable on disk, deleted once delivered. This is a privacy promise, not an optimisation.
- Webhook failures (decided 2026-10-25): the bridge confirms receipt to WhatsApp only after Rust replies 2xx (`AddEventHandlerWithSuccessStatus` returning `false`). Otherwise it reconnects with backoff and WhatsApp resends (kept up to 30 days). A resent message can't be decrypted twice, so the decrypted copy waits in a custom whatsmeow `EventBuffer` that encrypts it with a key from the environment and deletes it once Rust accepts. Rust dedupes on (chat, message ID). Rejected: memory-only retries (songs lost during deploys or crashes) and whatsmeow's default buffer (plain text on disk).
- One always-on connection per User: a hosting cost that grows with users.
- Linked devices drop if the phone stays offline too long; the app must detect and handle that.
