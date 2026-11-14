# 0012 — Derived Item identity: links and matches, no merges

Status: Accepted (2026-11-14); amended the same day after a schema review run on real data (`docs/reviews/2026-11-14-schema-review/`), which also deferred the Spotify expiry. Supersedes ADR-0006's `items`, `item_links`, `shares` and `listened` tables; the rest of ADR-0006 stands.

## Context

ADR-0006 stores an Item as a row. When a match reveals that two Items are the same recording, they must be merged, and a wrong merge needs an undo: `item_merges`, `merged_into` chains, `not_same` pairs, per-field restore (`docs/progress.md`). None of it was ever needed by real data. The 2026-11-10 external review proposed deriving identity instead. ADR-0011 makes that simple: a Song is one recording, which an ISRC identifies.

## Decision

No `items` table. The stored facts are:

- **`links`** — one track (later album…) on one service: unique `(service, kind, external_id)`, with its ISRC when known and its display metadata (title, artist, duration, cover). One `metadata_source` per link, not per field: the fields come together from one read, the Deezer API (`deezer_api`) or the WhatsApp preview (`preview`); a Deezer API read replaces a preview's fields as a group, never the reverse (decided 2026-11-14; MusicBrainz is out of the prototype, ADR-0008). Ranking sources per field happens across links, at display. Each metadata field may be NULL (unknown): a real Spotify preview had the title "open.spotify.com". Display data stays on the link it was read from, never copied to another.
- **`matches`** — a Match (glossary): `(link_id, service, matched_link_id, method, confidence, attempted_at)`, unique on `(link_id, service)`: "this link's recording on that service is that link". `confidence` keeps ADR-0006's ordered enum (`none < low < high < exact`, compared with `>=`). `method` becomes `text` + `check` with only `isrc`, `metadata`, `manual` (decided 2026-11-14): it has no order and will change, and an enum value can never be removed. `source` has no meaning without stored Items (a shared link is its own `links` row), `upc` waits for albums, `odesli` is deprecated (ADR-0005), `musicbrainz` is out of the prototype (ADR-0008); each comes back by replacing the `check`. "Searched, not found" is a row with `matched_link_id` NULL and `confidence` `none`, retried after ~7 days.
- **`shares`** — keep the URL as sent (`original_url`) and point to its link (`link_id`), NULL until the URL is resolved: a Deezer short link (`link.deezer.com/s/…`, 4 of spike A's first 9 links) has no ID until its redirect is followed. Unique on `(conversation_id, message_id, original_url)`, so a resent message is dropped and a message with two links gives two Shares. Plus the sender, message, `sent_at` and `received_at`. The Feed sorts on `sent_at`: history sync and catch-up deliver old messages late, so `received_at` would put them on top. `shares` holds no `account_id`: the Feed joins `conversations` and `chat_connections` to reach the Account, and an index on `shares (conversation_id, sent_at, id)` serves each conversation. A copied `account_id` (one index for the whole Feed) was rejected on 2026-11-14: a redundancy with no measured need at the prototype's scale; add it by migration if the Feed's sort ever shows up slow.
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
- Spotify links read from the Spotify API remain Spotify Content (ADR-0008); IDs taken from a shared link do not. A Spotify `links` row created from the API stores only its ID: no ISRC and no metadata from Spotify (the ISRC stays on the link the search started from). So the row itself holds nothing to expire; the Spotify Content is the Match pointing to it.
- **Spotify expiry deferred (decided 2026-11-14).** Only a User whose Preferred service is Spotify makes Quincy search Spotify, and the prototype's only User prefers Deezer, so no Spotify Content exists yet. When the first Spotify User joins, a migration adds a nullable `matches.expires_at`, and a job re-checks Matches before they expire; an expired Match is not served (rerun the matcher on open, else the search link). A `links` row something points to is never deleted, so Listened never blocks the expiry (schema review scenario S7). Until then the schema has no expiry column.
- Edge-case rules 4 to 8 in `docs/progress.md` (merge triggers, undo, cascades) are closed.
