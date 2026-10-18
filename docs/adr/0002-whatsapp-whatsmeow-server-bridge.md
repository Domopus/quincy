# 0002 — WhatsApp via whatsmeow in a server-side Go bridge

Status: Accepted (2026-10-18), amended same day: filtering moved to Rust; delivery by webhook.

## Context

Quincy must see links sent in all of the User's WhatsApp conversations, including muted ones. There is no official API for reading a personal account.

## Decision

A thin Go bridge on the server runs one whatsmeow linked-device session per User. It knows nothing about Quincy: no Shares, no Users, no matching.

- **Storage:** only whatsmeow's own session store (device keys), so restarts never force a re-pair. No Quincy data.
- **Incoming messages → webhook:** for every message, the bridge POSTs (user, sender, conversation, text, time) to the Rust server's `POST /internal/messages`, then forgets it. Rust filters for music URLs and drops everything else (ADR-0003). Connection lost/restored is sent the same way.
- **Commands → bridge API:** Rust calls the bridge to start pairing (returns the 8-char code), unlink, and get status.
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
- Decrypted messages pass from bridge to Rust in memory — keep both on the same machine or private network; never log or store message text beyond the music links and the message written with them. This is a privacy promise, not an optimisation.
- If Rust is down, messages are lost unless the bridge retries (see `docs/progress.md` Assumed).
- One always-on connection per User: a hosting cost that grows with users.
- Linked devices drop if the phone stays offline too long; the app must detect and handle that.
