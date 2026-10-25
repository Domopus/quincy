# 0009 — One contract for all chat bridges

Status: Accepted (2026-10-25)

## Context

WhatsApp is the first chat source; Messenger and others come later. None has an official API for reading a person's own chats (Meta's Messenger Platform and Instagram messaging API only serve business accounts), so each will need its own unofficial client, possibly in another language.

## Decision

- Every chat source is a thin **bridge**: it only talks to its chat platform. Link detection, matching, streaming services and the Feed stay in the Rust server, written once for all sources (ADR-0003).
- **Bridge → Rust:** one shared webhook and message format for every source: `source`, message ID, user, sender, conversation, text, link preview, time. The WhatsApp bridge sends `source` from day one.
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
