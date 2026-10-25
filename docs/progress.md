# Progress

_Rewrite this file; don't append. Git keeps the history._

## Where we are (2026-10-25)

Re-framing the project after a 6-month break; grilling the open questions. No new code yet. Decisions so far are in `docs/adr/` and `GLOSSARY.md`. Grilling session of 2026-10-25 settled: no username in the prototype, onboarding order, Spotify data rules (ADR-0008), Deezer + Spotify as Preferred services, chat bridge contract and delivery rules (ADR-0009), webhook failures (ADR-0002), Item naming (ADR-0006).

## Next step

Resume `/grilling` at **Item merging** (below): the proposal was put to the user on 2026-10-25, no answer yet. Then repo layout, then the grilling wrap-up (review Assumed, confirm shared understanding).

First task when the matcher is built (decided 2026-10-25): live-test that a new development-mode Spotify app can call Search with `isrc:` using client credentials. If not, Spotify output falls back to the search link.

First task when the matcher is built (decided 2026-10-25): live-test that a new development-mode Spotify app can call Search with `isrc:` using client credentials. If not, Spotify output falls back to the search link.

## Open questions (for grilling)

- **Repo layout** inside the monorepo (folder names, where generated clients live).
- **Item merging** (ADR-0006): when a match finds that two Items are the same. Proposal awaiting the user's answer: always merge into the older Item; the absorbed Item's rows lose `source` and take the match's `method`/`confidence`, so translation confidence stays honest; Shares and Listened move over (earliest Listened time kept); log each merge in `item_merges` for manual undo.
- **Unverified, check when the bridge exists:** how often WhatsApp previews are missing for Spotify links, and whether `musicMetadata` is filled for them.

## Streaming API facts (researched 2026-10-18)

Lookup (ISRC or search) and deep links need no user auth on any service. The binding limits are the terms of service, not missing features.

- **Spotify:** dev mode allows 5 authorised users, needs the owner to have Premium, and caps search at 10 results. Extended quota requires an organisation with 250k MAU. Plain `open.spotify.com` links work without the API.
- **Deezer:** public `GET /track/isrc:{ISRC}` needs no auth. Terms say non-commercial use only; commercial use needs a partnership.
- **Apple Music:** ISRC lookup and a universal `url` per song. Needs a US$99/yr developer token. The MusicKit clause forbids monetising access, e.g. ads around it.
- **Tidal:** v2 API has ISRC filter and Android/iOS deep links. Terms say non-commercial; production quota reportedly frozen (unverified).
- **YouTube Music:** no official API, only the YouTube Data API: search only, no ISRC, 100 searches/day. Stored data must be refreshed or deleted within 30 days.

Spotify terms (checked 2026-10-25, Developer Policy and Terms effective 2025-05-15):

- Policy §III: "Do not build an SDA that enables the transfer of data to another service", except a user's personal data or playlist metadata. Sending a Spotify track's title/artist to Deezer search is such a transfer.
- Precedent: in 2020 Spotify made SongShift remove Spotify → rival-service transfers or lose API access.
- oEmbed falls under the Developer Terms, the Widget Terms bar transfer "to another music service that competes with Spotify", and it returns only the title (no artist, no duration).
- The public track page's meta tags hold title, artist and duration, but automated reading is scraping, banned by Spotify's User Guidelines.
- ISRC: removed from dev-mode track objects in Feb 2026, restored in March 2026 (`external_ids` "[REVERTED]").
- No official Spotify statement on link converters; Odesli and Soundiiz still support Spotify, method unverified.

Deezer → Spotify direction (checked 2026-10-25):

- No Spotify clause bans looking up a Deezer song's ISRC in Spotify Search (`isrc:` filter still documented; `limit` max 10). Main risk is Policy §III "integrated with … content from another service", which Spotify has never defined.
- A stored Deezer ↔ Spotify ID mapping is likely "Spotify Content": Terms forbid storing it "indefinitely", so it would need a TTL / re-check. No retention period is given.
- App-only (client-credentials) access in development mode: the 5-user cap is tied to user tokens, so it likely doesn't apply, but a third party reports Spotify is phasing it out for metadata endpoints (unverified). Needs a live test.
- No-API fallback: `https://open.spotify.com/search/<artist title>` is a claimed Universal Link on iOS (verified in apple-app-site-association); Android untested.
- Deezer terms: nothing bans sending Deezer data to Spotify. Risks are the non-commercial clause (no "data or any other consideration"), no association with other brands, and "other purposes need approval".

Sources: official developer docs and terms for each service (Spotify policy and quota-modes, Deezer terms and FAQ, Apple DPLA §3.3, TIDAL OpenAPI spec, YouTube developer policies).

## whatsmeow delivery facts (researched 2026-10-25, whatsmeow HEAD `b2b1e71`)

- A handler registered with `AddEventHandlerWithSuccessStatus` that returns `false` stops whatsmeow sending the delivery receipt. The server keeps an unreceipted message and resends it on reconnect. WhatsApp keeps undelivered messages encrypted for up to 30 days (privacy policy).
- Catch: a resent message can't be decrypted twice (the Signal session has moved on), so it is silently dropped. whatsmeow's fix, `EnableDecryptedEventBuffer`, stores the plaintext on disk until the handler succeeds. Its `store.EventBuffer` interface can be replaced (e.g. encrypted).
- A blocked handler stalls all later events; after ~5 min whatsmeow continues in the background and ordering is lost. On failure: return `false` fast and reconnect, don't block. A panicking handler counts as success, so recover inside it.
- Message ID (`Info.ID`) is stable across resends; dedupe in Rust on (chat, ID). The sender may appear as a LID or a phone JID.
- Re-pairing triggers a history sync (`events.HistorySync`) with full messages incl. text/URLs; how far back is the phone's choice.
- Link preview fields on `ExtendedTextMessage`: `matchedText`, `title`, `description`, `JPEGThumbnail`, and `musicMetadata` (`author`, `title`, `songID`…; when it's filled is unverified). `canonicalURL` was removed.

## Design

- Figma file: https://www.figma.com/design/xbD2AKVjev41Xr7IjNoe3s/Quincy?node-id=45-4
- No full review now (decided 2026-10-18). When a question touches the UI (identity, pairing), extract only the relevant frames with `figma-extract-spec` and check them against the ADRs. The designs may predate the current spec.
- Create `DESIGN.md` when UI work starts.

## Assumed (review with the user)

- _(none right now)_

## Planned backend conventions (apply when the Rust server is scaffolded, then move to `backend/CLAUDE.md`)

- A shared `CrudService` trait for entity services — speeds up building new modules.
- `utoipa` for OpenAPI/Swagger docs on every controller; the generated spec is also the source for the TypeScript client (ADR-0001).

## Housekeeping done

- Removed committed `.claude/settings.local.json` (leftover personal permissions) and gitignored it.
