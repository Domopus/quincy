# 0006 — Catalogue data model: one row per (item, service)

Status: Accepted (2026-10-18). Names settled 2026-10-25: Item, `items`, `item_links`.

## Context

A link shared from any service must open on the User's service, every match should be computed once and reused for all Users, and albums (later) must fit without a redesign.

## Decision

- **`items`** — one row per piece of music, with a `kind` enum (`song`, `album`, `playlist`, `artist`; only `song` in the prototype). Holds display metadata (title, artist, cover, duration) and the cross-service code when known: ISRC for tracks, UPC for albums.
- **`item_links`** — `(item_id, kind, service, external_id, confidence, method, attempted_at)`. One row per service; unique on `(service, kind, external_id)`, because a service's IDs are only unique per kind (a Deezer track and a Deezer album can share a number). `kind` repeats the item's, kept consistent by a composite foreign key `(item_id, kind)` (corrected 2026-10-25). Rows for the same item share `item_id`.
- **`shares`** — `(user, contact, item_id, original_url, conversation, message, received_at)`, plus `(source, message_id)` from the bridge, unique per conversation so resent messages are dropped (ADR-0009).
- **`listened`** — `(user, item_id, listened_at)`: per User + item, not per Share.

### Lookup

Any incoming link → follow redirect, clean → look up `(service, external_id)` → self-join on `item_id` for the User's service. Store IDs, never URLs; URLs are built in code per (service, kind). Match as soon as a Share arrives, for the recipient's service.

### `method` (enum)

| Value | Meaning | Auto re-run |
|---|---|---|
| `source` | ID taken from the shared link that created this item | never |
| `isrc` | found on the target service by ISRC (tracks) | no |
| `upc` | found by UPC (albums) | no |
| `metadata` | artist + cleaned title, scored by duration (tracks) / track count + year (albums) | yes |
| `musicbrainz` | Spotify link with no WhatsApp preview, resolved by MusicBrainz (ADR-0008) | yes |
| `odesli` | second opinion when `metadata` is low | yes |
| `manual` | corrected by hand | never |

Matcher order: `isrc`/`upc` → `metadata` → `odesli` if `metadata` is low.

### `confidence` (enum, ordered `none < low < high < exact`)

Means "this ID belongs to this item" — the item's identity is anchored by its `source` row. A translation from service A to B has confidence `LEAST(A.confidence, B.confidence)`, never B's alone. Sharing a link that already exists does not upgrade its row to `source`.

Spotify rows are Spotify Content: they get an expiry and are re-checked, never kept forever (ADR-0008).

"Searched, not found" is stored as a row with `external_id` NULL, `confidence` `none`, and `attempted_at`; retried after ~7 days.

## Rejected

- **One column per service on the song** — every new service is a migration, and there's no place for per-service confidence.
- **A table per kind** (songs, albums) — duplicates the lookup flow; `kind` + per-kind matching rules is enough.

## Consequences

- Playlists can't be matched as a whole across services; they'd need track-by-track conversion (later).
- Open: how two items get merged when a match reveals they're the same.
