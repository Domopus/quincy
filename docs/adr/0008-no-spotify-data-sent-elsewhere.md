# 0008 — Spotify data never leaves for another service

Status: Accepted (2026-10-25)

## Context

To open a shared Spotify link on Deezer, the matcher (ADR-0005) needs the song's title, artist and ideally ISRC or duration. Spotify's Developer Policy (effective 2025-05-15) forbids an app "that enables the transfer of data to another service", except a user's own personal or playlist data. In 2020 Spotify made SongShift remove exactly this flow. oEmbed falls under the same terms, and reading the track page is scraping, banned by Spotify's User Guidelines. Facts and sources: `docs/progress.md`.

## Decision

- Quincy never takes data from Spotify (Web API, oEmbed or the track page) to query another service.
- For a shared Spotify link, song details come from the **WhatsApp link preview** the Contact's app attaches to the message (title; description "Artist · Album · Song · Year"). The bridge forwards it in the webhook.
- A link shared on the User's own Preferred service opens as-is: no matching.

## Rejected

- **Spotify Web API for metadata or ISRC** — the most accurate, but a "transfer of data to another service"; risks the API key and a rebuild before release.
- **oEmbed** — same terms, and it returns only the title.
- **Scraping the public track page** — banned by Spotify's User Guidelines.
- **Deezer links only** — drops half of the core loop.

## Consequences

- No duration from previews, so Spotify-link matches lean on artist, title and album: more "best guess" results (ADR-0005 handles them).
- The other direction is allowed by the written terms: a Deezer song's ISRC may be looked up in Spotify Search to build a Spotify link (prototype supports Deezer + Spotify, decided 2026-10-25). Stored Deezer ↔ Spotify mappings are Spotify Content: they get an expiry and are re-checked, never kept forever.
- Some messages arrive without a preview. Fallback order (decided 2026-10-25):
  1. WhatsApp preview.
  2. MusicBrainz lookup by the Spotify URL (free, CC0, 1 req/s; gives title, artist, duration, often ISRC). Partial coverage: 1 of 3 tracks in a quick test.
  3. Nothing found: the card shows who shared it, the conversation and their message, with a "Spotify song" placeholder. A Spotify User opens it as-is; others get **Open original** plus a search link on their service.
