# 0004 — No streaming write APIs for the prototype

Status: Accepted (2026-10-18)

## Context

The original idea was "Add to queue" on Spotify. Facts as of Oct 2026:
- Deezer has no queue/playback endpoint, and has refused new developer apps since ~Aug 2024 — so no official writes at all (not even playlists).
- Spotify (Feb 2026): new dev apps need Premium, 5 users max; extended access needs 250k MAU. ISRC removed from track objects.
- Deezer's public read API still works without an app (search, metadata, ISRC, previews).

## Decision

The Feed is the list. "Open in my app" opens the Song in the User's streaming app through a normal link (converted to their service). No queue, no playlist writes.

## Rejected

- **Deezer unofficial login cookie (ARL)** — works, but breaks Deezer's terms and risks the User's account. Possible later only as a personal opt-in experiment.
- **Spotify queue** — official, but capped at 5 users and the User is on Deezer.
- **Shazam-style auto playlist** — Shazam does it as an official partner; not available to us.

## Consequences

The app is service-agnostic from day one. Revisit if Deezer reopens app creation, or if Tidal / Apple Music official write APIs fit (open research item).
