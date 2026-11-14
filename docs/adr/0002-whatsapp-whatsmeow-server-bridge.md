# 0002 — WhatsApp via whatsmeow in a server-side Go bridge

Status: Accepted (2026-10-18), amended same day: filtering moved to Rust; delivery by webhook. Amended 2026-11-14: server-side confirmed for the prototype; reviewed before friends join.

## Context

Quincy must see links sent in all of the User's WhatsApp conversations, including muted ones. There is no official API for reading a personal account.

## Decision

A thin Go bridge on the server runs one whatsmeow linked-device session per User. It knows nothing about Quincy: no Shares, no Users, no matching.

- **Storage:** only whatsmeow's own session store (device keys; a SQLite file outside the repo, pure-Go driver, decided 2026-11-14), so restarts never force a re-pair, plus the encrypted buffer of messages Rust hasn't accepted yet (see Consequences). No Quincy data.
- **Incoming messages → webhook:** for every message, the bridge POSTs (source, user, sender, conversation, text, link preview, time; shared format per ADR-0009) to the Rust server's `POST /internal/messages`, then forgets it. Rust filters for music URLs and drops everything else (ADR-0003). Connection lost/restored is sent the same way.
- **Commands → bridge API:** Rust calls the bridge to start pairing (returns the 8-char code), unlink, and get status. The bridge also reports a logout by WhatsApp to Rust, which prompts the User to reconnect (added 2026-10-25).
- **Contracts (ADR-0001):** Rust owns the webhook endpoint (`utoipa` → generated Go client, `oapi-codegen`); Go owns the commands API (`huma` → generated Rust client, `progenitor`). The bridge is configured with Rust's URL and a shared secret.

## Server or phone (2026-11-14)

The external review (`docs/reviews/2026-11-10-fable-review.md`) proposed running the session on the phone, inside the Quincy app. Kept on the server for the prototype:

- The bridge and webhook exist and run spike A; a phone session means rewriting the bridge into the app (gomobile or whatsapp-rust) and moving link filtering out of Rust (ADR-0003).
- With one User, the developer, on the developer's own machine, the phone's gains (messages never leave the device, home IP) are already met.
- A server session leaves the client free: a list of cards with links out needs no native code. A phone session forces Tauri or native, a foreground service on Android, and sync only while open on iOS.

Lost: privacy does not hold by construction (the whatsmeow session store stays the asset to protect), and the reply assistant (product.md "Later"), which reads every message, would be easier to defend on the phone. Both matter once friends join, so the choice is **reviewed before friends join**, with spike A's catch-up numbers (a Mac that sleeps behaves like a phone).

## Rejected

- **Android notification listener (on device)** — private and no ban risk, but misses muted chats and doesn't exist on iOS.
- **whatsapp-rust** — very active but pre-1.0 (v0.7, Aug 2026). Revisit if it matures; it would remove Go.
- **Baileys (Node)** — independent reimplementation, more breakage reports, heavier per connection.
- **mautrix-whatsapp** — needs a Matrix homeserver, which stores every bridged message as room events: breaks the privacy promise (only music links are kept), and adds a server plus a Matrix account per User. Its network connector without the homeserver is a separate option, open in `docs/progress.md` (added 2026-11-01).
- **Filtering in the bridge** — would put logic in Go; Rust holds all logic (ADR-0003), and the Go part stays easy to replace.
- **Rust polling a GET** — the bridge would have to keep private messages waiting until fetched.
- **SSE stream from the bridge** — works (Go `huma` documents it), but SSE is idiomatic for server → browser, not service → service; Rust's `progenitor` has no SSE client, so the reader would be hand-written. Webhooks are the conventional pattern (also used by whatsmeow REST wrappers like wuzapi) and fully generated.

## Consequences

- Unofficial client: WhatsApp can ban the User's number. Quincy only reads, which is the low-risk end; the prototype never sends messages. Sending replies the User approved is a later goal (product.md "Reply assistant", added 2026-11-14); it would raise the ban risk and needs a send command on the bridge.
- Decrypted messages pass from bridge to Rust in memory — keep both on the same machine or private network; never log or store message text beyond the music links and the message written with them. The only exception is the bridge's encrypted buffer: never readable on disk, deleted once delivered. This is a privacy promise, not an optimisation.
- Webhook failures (decided 2026-10-25): the bridge confirms receipt to WhatsApp only after Rust replies 2xx (`AddEventHandlerWithSuccessStatus` returning `false`). Otherwise it reconnects with backoff and WhatsApp resends (kept up to 30 days). A resent message can't be decrypted twice, so the decrypted copy waits in a custom whatsmeow `EventBuffer` that encrypts it with a key from the environment and deletes it once Rust accepts. Rust dedupes on (chat, message ID). Rejected: memory-only retries (songs lost during deploys or crashes) and whatsmeow's default buffer (plain text on disk).
- One always-on connection per User: a hosting cost that grows with users.
- Linked devices drop if the phone stays offline too long; the app must detect and handle that.
