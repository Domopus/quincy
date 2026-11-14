# Quincy

Mobile app that collects the songs friends share in chat apps (WhatsApp first) into one feed, with a button to open each song in your own streaming app.

## Read first

- `docs/progress.md` — where we are and the next step. Read at the start of every session.
- `GLOSSARY.md` — domain terms. Use these words, not synonyms.
- `docs/product.md` — product vision; read when a product question comes up.
- `docs/adr/` — decisions and rejected alternatives; read the ones touching your task.

## Working rules

- Settle a decision → update the docs in the same turn: `progress.md`, the glossary, an ADR if it's hard to reverse, surprising, and the result of a real trade-off.
- A doc changes in the same commit as the code it describes. When a decision is reversed, mark its ADR `Superseded` — never leave it stale.
- Don't re-propose a rejected alternative without new facts; the ADR says why it was rejected.

## Key decisions

- Plain-folder monorepo, no monorepo tool (ADR-0001).
- WhatsApp via whatsmeow in a thin Go bridge on the server; it POSTs every message to Rust by webhook and stores no Quincy data (ADR-0002).
- Rust server holds all logic; Go only for the bridge (ADR-0003).
- No streaming write APIs for the prototype: songs open in the user's app via links (ADR-0004).
- Cross-service song matching is our own code: ISRC first, then cleaned metadata + duration (ADR-0005).
- Catalogue: one row per (item, service); translation confidence = the weaker of the two rows (ADR-0006).
- Identity is an email Account; chat sources are Chat connections attached to it; streaming is only a Preferred service, no login (ADR-0007).
- Spotify data never goes to another service; Spotify-link details come from the WhatsApp link preview (ADR-0008).
- Every chat source is a thin bridge sharing one webhook format (with `source`) and one command set (ADR-0009).
- One folder per deployable; specs beside their owner; all generated code committed; one Rust package until the Tauri core (ADR-0010).
- Deferred on purpose for the prototype: authorization, token encryption, Tauri-specific features. Not bugs.

## State of the code

Layout in ADR-0010: `server/` (Rust), `bridges/<source>/` (Go), `app/` (Tauri, later). Commands:

- `mise install` — install Go, Rust and sqlx-cli at the pinned versions.
- `mise run db` — start the dev Postgres (OrbStack or Docker).
- `mise run secrets` — create `.env` with random `BRIDGE_SECRET` and `SENDER_HASH_KEY`, once. Never commit it: a known hash key lets anyone recover senders by hashing every phone number.
- `mise run dev` — run the server on `127.0.0.1:8080`; migrations run at start.
- `mise run gen` — regenerate `server/openapi.json` and the sqlx offline cache (`server/.sqlx/`). Run after changing an endpoint or a query.
- `mise run check` — fmt, clippy (`-D warnings`), tests. Integration tests use `#[sqlx::test]` (one throwaway database per test). Generated specs and clients are committed and regenerated with `mise run gen`; never edit them by hand.
