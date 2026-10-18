# 0002 — WhatsApp via whatsmeow in a server-side Go bridge

Status: Accepted (2026-10-18)

## Context

Quincy must see links sent in all of the User's WhatsApp conversations, including muted ones. There is no official API for reading a personal account.

## Decision

A thin Go service on the server runs one whatsmeow linked-device session per User. It filters every incoming message for music URLs, forwards those (plus the message text and sender/conversation info) to the Rust server, and drops everything else immediately.

## Rejected

- **Android notification listener (on device)** — private and no ban risk, but misses muted chats and doesn't exist on iOS.
- **whatsapp-rust** — very active but pre-1.0 (v0.7, Aug 2026). Revisit if it matures; it would remove Go.
- **Baileys (Node)** — independent reimplementation, more breakage reports, heavier per connection.
- **mautrix-whatsapp** — needs a Matrix homeserver.

## Consequences

- Unofficial client: WhatsApp can ban the User's number. Quincy only reads, which is the low-risk end; never send messages.
- The server sees decrypted messages in memory — the filter-and-drop rule is a privacy promise, not an optimisation.
- One always-on connection per User: a hosting cost that grows with users.
- Linked devices drop if the phone stays offline too long; the app must detect and handle that.
