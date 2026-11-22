# 0014 — Spotify Web API for the ISRC, personal prototype only

Status: Accepted (2026-11-22), **blocked the same day**: every Spotify Web API call (`GET /tracks/{id}`, Search `isrc:`) answers 403 "Active premium subscription required for the owner of the app", although the client-credentials token is issued. The developer will not pay Premium for this (2026-11-22), so until then nothing is read from Spotify and the fallback order stays ADR-0008's (WhatsApp preview → placeholder). Partly supersedes ADR-0008 (its first rule, for the prototype only) once unblocked.

## Context

ADR-0008 forbids taking data from Spotify to query another service: Spotify's Developer Policy bans "the transfer of data to another service", and the stated risk was losing the API key and rebuilding before release. Two facts are new since:

- Spike A (2026-11-14 → 2026-11-22): 3 of 10 shared Spotify links had no usable WhatsApp preview (one none, two with the title `open.spotify.com`), so they could not be matched at all. Spike B found 6 of 7 previews matched on Deezer, but without duration a cleaned title can hit a live take instead of the studio one ("Latinoamérica", Calle 13).
- Quincy is a personal tool for now (`product.md` "Audience"), heading towards Plugins (`product.md` "Direction"). The only key at stake is the developer's own Spotify app, and the only User is the developer.

## Decision

- For the personal prototype, the music code reads a shared Spotify link's ISRC from the Spotify Web API (`GET /tracks/{id}`, client credentials of the developer's own Spotify app) and matches on Deezer by ISRC first (ADR-0005 step 1).
- Only the ISRC is taken. Display metadata still comes from the WhatsApp preview, or from the matched Deezer link.
- The ISRC is stored on the Spotify `links` row. It is Spotify Content: this ADR does not change the deferred expiry (ADR-0012).
- All Spotify Web API calls live in one module of the music code, so they can be removed in one place.

## Rejected

- **Keep ADR-0008 as is** — loses 30% of spike A's Spotify links and leaves version traps unsolved, to protect a key that is the developer's own.
- **Take full metadata (title, artist, duration) from Spotify too** — not needed: the ISRC alone gives an exact Deezer match, whose API brings the metadata. Reopen if ISRC matches fail.
- **Ship it to other Users unchanged** — the policy risk then falls on Quincy, not on one developer's key.

## Consequences

- Before anyone but the developer uses Quincy, this ADR is reviewed: either the Spotify module becomes a Plugin that runs on the User's own Spotify credentials, or ADR-0008's rule comes back. Shipping a Policy-breaking Plugin from a Quincy marketplace does not fully move the responsibility to the User.
- The bridge still forwards the preview; ADR-0008's fallback order becomes: Spotify ISRC → WhatsApp preview → placeholder.
- Needs `SPOTIFY_CLIENT_ID` and `SPOTIFY_CLIENT_SECRET` in `.env`, never committed.
