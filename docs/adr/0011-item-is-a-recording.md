# 0011 — A Song Item is one recording

Status: Accepted (2026-11-14).

## Context

The same song exists in many versions: "Come Together" as the 1969 original, the 2009 remaster and the 2019 mix (three ISRCs); "Without Me" as explicit, clean and radio edit (one title). `items` (ADR-0006) needs one identity rule before `shares` and `items` are designed, and the Feed depends on it.

## Decision

A Song Item is one recording: the level an ISRC identifies. A remaster, a mix, a live take, a mono version or a clean/explicit edit is a different Item. "Open in my app" opens that recording, as `product.md` "Prototype success" already requires.

## Rejected

- **A song in any version (the work)** — one Feed card per song, but no reliable identifier: services rarely expose the ISWC, and MusicBrainz groups versions incompletely and sometimes wrongly (tests of 2026-11-03 in `docs/progress.md`). Grouping would rest on cleaned titles and artists, with more wrong merges, and would still need a rule for which version to open.

## Consequences

- The Feed can show two cards for what the User hears as the same song. Grouping them on screen stays possible later without changing identity.
- Listened stays per recording: listening to the remaster doesn't mark the original.
- A `metadata` match (no ISRC) must not link two versions of a song; how it tells them apart is still open (version tags, see `docs/progress.md`).
- Open, for when albums start: the same rule would make an album Item one edition, the level a UPC identifies (standard and deluxe as two Items).
