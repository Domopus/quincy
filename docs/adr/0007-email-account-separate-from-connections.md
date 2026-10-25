# 0007 — Email Account, separate from Chat connections

Status: Accepted (2026-10-18)

## Context

The old spec used Spotify login as identity. ADR-0004 removed streaming logins, so something else must say whose Feed a Share belongs to. Quincy will gain more chat sources (WhatsApp, then Messenger, maybe Instagram).

## Decision

- A User signs up with **email + password**, verified by a 6-digit code sent by email and typed in the app. That is the Account. A code works whichever device the mail is read on, with no App Links setup.
- Chat sources are **Chat connections** attached to the Account. Each one is a live session (WhatsApp pairing first).
- Streaming is only a **Preferred service**: a choice of where songs open. No streaming login (ADR-0004); song lookups use app-level access.
- Onboarding after verification: Preferred service, then Chat connections (revised 2026-10-25: the one-tap step goes before the pairing step, where people drop off). Username dropped from the prototype on 2026-10-25; see product.md "Later".

## Rejected

- **WhatsApp phone number as identity.** No extra signup step, but each new chat source would bring its own identity to link, a new phone number would mean a new account, and reinstalling the app would mean pairing again.
- **Anonymous device-bound account.** The Feed is lost with the phone.
- **Streaming login as identity.** Ruled out by ADR-0004, and Spotify dev mode caps an app at 5 users.

## Consequences

The server needs sending mail and password storage from the start. Google/Facebook SSO can be added later as other ways to sign in to the same Account.
