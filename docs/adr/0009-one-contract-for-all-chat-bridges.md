# 0009 — One contract for all chat bridges

Status: Accepted (2026-10-25)

## Context

WhatsApp is the first chat source; Messenger and others come later. None has an official API for reading a person's own chats (Meta's Messenger Platform and Instagram messaging API only serve business accounts), so each will need its own unofficial client, possibly in another language.

## Decision

- Every chat source is a thin **bridge**: it only talks to its chat platform. Link detection, matching, streaming services and the Feed stay in the Rust server, written once for all sources (ADR-0003).
- **Bridge → Rust:** one shared webhook and message format for every source: `source`, message ID, user, sender, conversation (ID, `direct`/`group`, name), text, link preview, time. The WhatsApp bridge sends `source` from day one. Implemented 2026-11-14 in `server/openapi.json`: "user" is `connection_id` (the Chat connection; the bridge knows no Users), and `origin` (`live`, `catch_up`, `history_sync`) was added so spike A can tell live messages from resends and history. The webhook answers 204 when stored, 401 on a wrong bridge secret, 500 when not stored (the bridge must not confirm receipt).
- **Which Chat connection a message belongs to** (decided 2026-11-14, after comparing mautrix, WAHA, Evolution API, WuzAPI, GOWA, GitHub Apps, Slack and Stripe):
  1. `connection_id` is `chat_connections.id`. Rust creates the row when the User starts pairing and passes its id in the pairing command; the bridge keeps its session under that id and sends it in every webhook. Same pattern as the WhatsApp multi-session APIs, which all create the session before pairing.
  2. On pair success the bridge reports the account's own ID (the WhatsApp JID). Rust stores only its hash (`chat_connections.external_hash`, same key as senders), unique per source: a number already linked to another Account is refused.
  3. A logout (unlinked from the phone, or by WhatsApp) marks the row logged out (`logged_out_at`) and never deletes it; re-pairing reuses the same id, and a different number replaces the hash. Only an explicit unlink deletes the row.
  4. An unknown `connection_id` (a deleted row) gets 404: the bridge acknowledges, logs and drops the message, with no retry.
  Rejected: keying on the JID (exists only after pairing and not after a logout; GOWA had to add a session id for this reason) and a bridge-assigned ID mapped by Rust at pair success (the GitHub/Slack install pattern, built for untrusted consumers; it adds a link step to secure). `connection_id` becomes an integer in the contract with the real webhook, after spike A's week.
- **Rust → bridge:** the same command set for every source: start pairing, unlink, status. Each bridge implements it its own way.
- Messages are pushed, never fetched (ADR-0002).
- **Delivery rules** every bridge must meet, each in its own way (added 2026-10-25):
  1. A message is never lost because Rust was down.
  2. Anything kept while waiting is encrypted, and deleted once Rust accepts it.
  3. Every message carries an ID that stays the same if it's sent again; Rust drops duplicates on (source, conversation, message ID).
  The WhatsApp bridge's way of meeting them is in ADR-0002.

## Rejected

- **A contract per source** — each new source would mean new Rust endpoints and a contract rework.
- **Rust fetching messages from bridges** — bridges would have to hold private messages until fetched (ADR-0002).

## Consequences

- Adding a source means a new bridge and no new matching logic.
- The shared format must stay generic: source-specific fields go in an optional per-source part, not the core.
