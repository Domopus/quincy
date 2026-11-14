# 0012 — Derived Item identity: links and matches, no merges

Status: Accepted (2026-11-14). Supersedes ADR-0006's `items`, `item_links`, `shares` and `listened` tables; the rest of ADR-0006 stands.

## Context

ADR-0006 stores an Item as a row. When a match reveals that two Items are the same recording, they must be merged, and a wrong merge needs an undo: `item_merges`, `merged_into` chains, `not_same` pairs, per-field restore (`docs/progress.md`). None of it was ever needed by real data. The 2026-11-10 external review proposed deriving identity instead. ADR-0011 makes that simple: a Song is one recording, which an ISRC identifies.

## Decision

No `items` table. The stored facts are:

- **`links`** — one track (later album…) on one service: unique `(service, kind, external_id)`, with its ISRC when known and its display metadata (title, artist, duration, cover), each with its source. Display data stays on the link it was read from, never copied to another.
- **`matches`** — a Match (glossary): `(link_id, service, matched_link_id, method, confidence, attempted_at)`, unique on `(link_id, service)`: "this link's recording on that service is that link". `method` and `confidence` keep ADR-0006's enums. "Searched, not found" is a row with `matched_link_id` NULL and `confidence` `none`, retried after ~7 days.
- **`shares`** — point to the shared link (`link_id`), exactly what the Contact sent; plus the conversation, sender, message and `(source, message_id)` as in ADR-0006.
- **`listened`** — `(user, link_id, listened_at)`, on the link of the User's Preferred service, the one they opened.

"Same Song" is computed, never stored: same ISRC, or a Match at `high` or above. Nothing is merged, so nothing is undone; a wrong Match is one row to delete.

Lookup: shared link → its `links` row (created if new) → its Match for the User's Preferred service → if none, run the matcher (ADR-0005), which creates the target `links` row and the Match.

Duplicates are tolerated in the prototype: the Feed shows one card per Share, and two Shares of the same Song from different services both open, and show as Listened on, the same link of the User's service.

## Rejected

- **Stored Items with merges (ADR-0006)** — a merge can hide a Share when wrong, and undoing it needs the whole merge machinery. Two cards for one Song is only cosmetic.

## Consequences

- Listened works across services for free: Shares from Spotify and Deezer of one recording translate to the same link of the User's service.
- No stable Song ID. A feature that counts Songs (e.g. "most shared") groups links at query time, or a stored Song table is added later by migration from `links` and `matches`.
- A Match's confidence is the row's own; ADR-0006's "weaker of the two rows" rule and `source` anchoring go away.
- Spotify links read from the Spotify API remain Spotify Content with an expiry (ADR-0008); IDs taken from a shared link do not.
- Edge-case rules 4 to 8 in `docs/progress.md` (merge triggers, undo, cascades) are closed.
