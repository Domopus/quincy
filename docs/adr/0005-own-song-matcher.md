# 0005 — Cross-service song matching is our own code

Status: Accepted (2026-10-18)

## Context

A Spotify link must open on Deezer (and later other services). Titles and album names differ across services ("Remastered 2011", "feat.", brackets).

## Decision

Matching lives in the Rust server:
1. Use the ISRC when we have it (Deezer links provide it).
2. Otherwise search the target service by artist + cleaned title.
3. Score candidates with duration (±3 s) as the strongest signal; album only breaks ties.
4. Store every result with a confidence level and method, reused for all Users. Low confidence → show "best guess" plus a search link, never a silent wrong song.

Odesli/song.link may be used as a second opinion when available.

## Rejected

- **Odesli as the main source** — free tier 10 req/min, API namespace deprecated, no new keys issued.
- **Reusing linkhop / musiclinkii code** — linkhop has no licence (can't copy); musiclinkii is GPL (would force GPL). Their approach is reused, not their code.

## Consequences

Spotify links have no ISRC since Feb 2026, so they always go through metadata matching.
