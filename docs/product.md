# Product

## Problem

Friends share songs in chat apps. They get buried in conversations and never listened to.

## Onboarding (prototype)

1. Sign up with email + password.
2. A screen says a 6-digit code was emailed; the User types it in. The code is valid 15 minutes and can be resent once a minute. The screen also offers **Resend email** and **Change email**, so it never dead-ends.
3. Choose a Preferred service (Deezer or Spotify): one tap, an easy win before the hard step.
4. Add a Chat connection (WhatsApp pairing), which ends on the Feed where the first Shares appear.

No username in the prototype: no screen shows one.

## Core loop (prototype)

1. A Contact sends a music link (Spotify or Deezer) in a WhatsApp conversation.
2. Quincy detects the link and creates a Share in the User's Feed. Everything else in the message stream is discarded.
3. Each post shows: who shared it, in which conversation, the message they wrote with it (if any), and the Song (cover, title, artist).
4. **Open in my app** opens the Song in the User's streaming app, converted to that service if needed. Opening marks the Share Listened.
5. The Feed filters Listened / Not listened; the User can toggle the status by hand.

Out of the prototype: "Add all to queue" and writing to a playlist — no legitimate API allows it today (ADR-0004).

## Targets

- **Platforms:** Android (GrapheneOS first), then iOS. Built with Tauri.
- **Chat sources:** WhatsApp → Messenger, others later.
- **Streaming services:** Deezer and Spotify, both as shared links and as Preferred service. Apple Music, Tidal, YouTube Music later (limits in `docs/progress.md`).
- **Share types:** songs first; albums, playlists, artists later.

## Principles

- **Privacy:** only music links (and the message written with them) are kept; everything else is dropped as soon as the server has read it. A message waiting for delivery is never stored in readable form.
- **Easy setup:** connecting WhatsApp is a ~30-second step. Pairing survives our restarts and updates; the User re-pairs only when WhatsApp itself ends the link (phone unused 14 days, or Quincy removed from Linked devices). Then a push notification and a **Reconnect WhatsApp** banner on the Feed rerun the same code flow.
- **Extensible:** each chat source and streaming service is an isolated module.

## Later (nice to have, not in the prototype)

- Sign in with Google / Facebook (SSO).
- A tap-to-open link in the verification email, alongside the code.

- Show every Contact who shared the same Song ("Alice and Bob shared this").
- Profile page: the songs the User has shared.
- Username, arriving with the profile page. Never empty: every Account gets a random default (e.g. `quiet-otter-17`) the User can change. Not derived from the email, which would expose part of the address.
- Albums, playlists, artists as share types.
- Admin tool to correct wrong matches (until then: edit the DB by hand).
