# Progress

_Rewrite this file; don't append. Git keeps the history._

## Where we are (2026-10-18)

Re-framing the project after a 6-month break. No new code yet. Decisions so far are in `docs/adr/` and `GLOSSARY.md`.

## Next step

Grill the open questions below with `/grilling` (global skill, built 2026-10-18), writing each answer into the docs as it lands.

## Open questions (for grilling)

- **Repo layout** inside the monorepo (folder names, where generated clients live).
- **User identity**: the old spec used Spotify login as identity; no streaming OAuth now. What replaces it?
- **WhatsApp pairing UX**: phone number → 8-char code in the app; confirm flow and re-pair handling.
- **DB details** (model agreed in ADR-0006): final table names; how two items get merged when a match reveals they're the same.
- **Streaming services to support**: research Tidal / Apple Music / YouTube Music official playlist-write APIs.
- **Design**: create `DESIGN.md` when the UI work starts.

## Assumed (review with the user)

- Bridge retries a failed webhook with backoff for a few minutes, in memory only, then drops the message — never writes message text to disk.

## Planned backend conventions (apply when the Rust server is scaffolded, then move to `backend/CLAUDE.md`)

- A shared `CrudService` trait for entity services — speeds up building new modules.
- `utoipa` for OpenAPI/Swagger docs on every controller; the generated spec is also the source for the TypeScript client (ADR-0001).

## Housekeeping done

- Removed committed `.claude/settings.local.json` (leftover personal permissions) and gitignored it.
