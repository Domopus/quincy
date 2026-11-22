# 0014 — Spotify embed page for shared links, personal prototype only

Status: Accepted (2026-11-22), revised the same day: first the Spotify Web API for the ISRC, blocked by Premium, then the embed page. Partly supersedes ADR-0008 (its first rule, for the prototype only).

## Context

ADR-0008 forbids taking data from Spotify to query another service: Spotify's Developer Policy bans "the transfer of data to another service", and its User Guidelines ban reading the track page. Facts that are new since then:

- Spike A (2026-11-14 → 2026-11-22): 3 of 10 shared Spotify links had no usable WhatsApp preview (one none, two with the title `open.spotify.com`), so they could not be matched. Spike B matched 6 of the 7 previews on Deezer. But without a duration, a cleaned title can hit a live take instead of the studio one ("Latinoamérica", Calle 13).
- Quincy is a personal tool for now (`product.md` "Audience"), heading towards Plugins (`product.md` "Direction").
- **Web API blocked:** since February 2026, a development-mode app only works if its owner has Premium. With valid client credentials (the token is issued), `GET /tracks/{id}` and Search `isrc:` both answer 403 "Active premium subscription required for the owner of the app" (14 of 14 calls, 2026-11-22). The developer will not pay Premium for this.
- **Embed page readable:** `open.spotify.com/embed/track/<id>` needs no key. Its `__NEXT_DATA__` JSON gave title, artists, duration (ms), release date and explicit flag for 4 of 4 tested links, including the 3 without a usable preview. It has no ISRC.

## Decision

- For the personal prototype, the music code reads a shared Spotify track link's metadata from its embed page: title, artists, duration, release date. The ID comes from the URL; a `spotify.link` short link is resolved first by following its redirect.
- Order: embed page → WhatsApp preview → placeholder (ADR-0008 step 3). The preview is kept for when the embed fails (changed format, blocked, track removed).
- Matching on Deezer uses artist + title + duration (ADR-0005 steps 2–3); there is no ISRC on this side.
- Stored on the Spotify `links` row with `metadata_source` `spotify_embed`. That needs a migration adding the value to `links_metadata_source_check`, made with the matcher. An embed read replaces a preview's fields as a group, never the reverse. A Deezer API read on a Deezer link is unaffected (ADR-0012).
- All reads from Spotify live in one module of the music code, so they can be removed in one place.

## Rejected

- **Spotify Web API (ISRC)** — needs the app owner to have Premium (403 otherwise). Using a Premium friend's app was also rejected: it depends on someone else's subscription, and development mode is capped at 5 users.
- **Odesli (song.link)** — answers 401 without a key since 2026, and its `v1-alpha.1` API is deprecated. A free key can be requested by email: a candidate source for the ISRC later, not a dependency.
- **Paid ISRC scrapers (Apify)** — about $0.005 per track, a third party between us and the data, and Spotify's API behind them anyway.
- **The web player's internal API** (librespot, spotDL) — Spotify hardens it regularly; too fragile.
- **Keep ADR-0008 as is** — loses 30% of spike A's Spotify links and leaves version traps unsolved.

## Consequences

- This reads a Spotify page, which Spotify's User Guidelines ban. It is accepted for a personal tool. Before anyone but the developer uses Quincy, this ADR is reviewed: the Spotify module becomes a Plugin the User chooses to install, or ADR-0008's rule comes back. A Policy-breaking Plugin shipped from a Quincy marketplace does not fully move the responsibility to the User.
- The page format is undocumented and can change without notice: parsing fails soft (fall back to the preview) and is logged.
- From a server's datacenter IP, the embed may be blocked; check it at the server-or-phone review (ADR-0002).
- The Deezer → Spotify direction is still blocked (Search needs the Web API): a Spotify User gets a search link only.
- Embed data is Spotify Content. Its expiry is deferred like the rest (ADR-0012).
