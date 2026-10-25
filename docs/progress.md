# Progress

_Rewrite this file; don't append. Git keeps the history._

## Where we are (2026-10-18)

Re-framing the project after a 6-month break. No new code yet. Decisions so far are in `docs/adr/` and `GLOSSARY.md`.

## Next step

Grill the open questions below with `/grilling` (global skill, built 2026-10-18), writing each answer into the docs as it lands.

## Open questions (for grilling)

- **Repo layout** inside the monorepo (folder names, where generated clients live).
- **Username**: who sees it, and is it unique?
- **Onboarding order**: username → Chat connections → Preferred service (ADR-0007), open to a better UX order.
- **WhatsApp pairing UX**: phone number → 8-char code in the app; confirm flow and re-pair handling.
- **DB details** (model agreed in ADR-0006): final table names; how two items get merged when a match reveals they're the same.
- **Streaming services to support**: which output services for the prototype. Research done 2026-10-18 (facts below); decision still open.
- **Spotify terms vs ADR-0005**: Spotify's Developer Policy (since 2025-05-15) says "do not create any product or service which is integrated with streams or content from another service". Resolving a Spotify link to an ISRC through the Spotify API for cross-service matching may conflict with it. Check this before building the matcher.

## Streaming API facts (researched 2026-10-18)

Lookup (ISRC or search) and deep links need no user auth on any service. The binding limits are the terms of service, not missing features.

- **Spotify:** dev mode allows 5 authorised users, needs the owner to have Premium, and caps search at 10 results. Extended quota requires an organisation with 250k MAU. Plain `open.spotify.com` links work without the API.
- **Deezer:** public `GET /track/isrc:{ISRC}` needs no auth. Terms say non-commercial use only; commercial use needs a partnership.
- **Apple Music:** ISRC lookup and a universal `url` per song. Needs a US$99/yr developer token. The MusicKit clause forbids monetising access, e.g. ads around it.
- **Tidal:** v2 API has ISRC filter and Android/iOS deep links. Terms say non-commercial; production quota reportedly frozen (unverified).
- **YouTube Music:** no official API, only the YouTube Data API: search only, no ISRC, 100 searches/day. Stored data must be refreshed or deleted within 30 days.

Sources: official developer docs and terms for each service (Spotify policy and quota-modes, Deezer terms and FAQ, Apple DPLA §3.3, TIDAL OpenAPI spec, YouTube developer policies).

## Design

- Figma file: https://www.figma.com/design/xbD2AKVjev41Xr7IjNoe3s/Quincy?node-id=45-4
- No full review now (decided 2026-10-18). When a question touches the UI (identity, pairing), extract only the relevant frames with `figma-extract-spec` and check them against the ADRs. The designs may predate the current spec.
- Create `DESIGN.md` when UI work starts.

## Assumed (review with the user)

- Email verification code: 6 digits, valid 15 minutes, resend allowed once a minute. Standard values, easy to change.
- Bridge retries a failed webhook with backoff for a few minutes, in memory only, then drops the message — never writes message text to disk.

## Planned backend conventions (apply when the Rust server is scaffolded, then move to `backend/CLAUDE.md`)

- A shared `CrudService` trait for entity services — speeds up building new modules.
- `utoipa` for OpenAPI/Swagger docs on every controller; the generated spec is also the source for the TypeScript client (ADR-0001).

## Housekeeping done

- Removed committed `.claude/settings.local.json` (leftover personal permissions) and gitignored it.
