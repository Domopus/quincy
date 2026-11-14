# 0010 — Repo layout: one folder per deployable, specs beside their owner

Status: Accepted (2026-11-14)

## Context

ADR-0001 chose plain folders but left the layout open. The April scaffold (`backend/`, `frontend/`) was built on an old spec. The first code is the server's webhook and the WhatsApp bridge, run for a week to measure real messages (spike A in `docs/progress.md`). Six reference repos were studied on 2026-11-14 (ente, immich, spacedrive, zed, mattermost, atuin).

## Decision

```
quincy/
├── server/                Rust package quincy-server (lib + bin)
│   ├── src/
│   ├── migrations/        sqlx, embedded with sqlx::migrate!()
│   ├── tests/             integration tests against a real Postgres
│   └── openapi.json       webhook + app API spec (utoipa), committed
├── bridges/
│   └── whatsapp/          Go module, binary quincy-whatsapp
│       ├── cmd/           quincy-whatsapp/ (bridge), openapi/ (spec exporter)
│       ├── internal/      session/, webhook/, commands/, quincyapi/ (generated)
│       └── openapi-3.0.json  command spec (huma), committed
├── app/                   Tauri (src/ TS, src-tauri/ Rust), created when needed
├── docs/
├── mise.toml              tool versions + tasks: gen, check, db, dev
├── compose.yaml           dev Postgres
└── .github/workflows/ci.yml
```

- **Naming:** a bridge folder is named after its webhook `source` value (ADR-0009). Rust packages are `quincy-<role>`. Go modules are `github.com/Domopus/quincy/bridges/<source>`, binaries `quincy-<source>`.
- **Specs live beside the API that owns them**, not in a shared folder (ADR-0001). The bridge's spec is committed in its 3.0 form, the one progenitor reads, so building Rust never needs Go.
- **All generated code is committed:** the Go client in `internal/quincyapi/`, the Rust client of the command API, later the TS client in `app/`. One rule for every language, and the agent can read both sides of each API. `mise run gen` regenerates every spec and client; CI fails if `mise run gen && git diff --exit-code` shows a change.
- **One Rust package, no workspace.** The root Cargo workspace (with `[workspace.dependencies]`) arrives with the Tauri core, the only reason to split a crate. No `common` crate.
- **No shared Go module** until a second Go bridge exists; then `bridges/contract/` plus a committed `go.work`.
- **mise** pins the tools (Go, Rust, sqlx-cli, oapi-codegen) and holds the tasks; ADR-0001 allows it.
- **Spike code goes straight into this layout.** There is no throwaway folder: the contract is settled (ADR-0002, 0009), and a throwaway would build whatsmeow sessions and the encrypted buffer twice.

## Rejected

- **A root `openapi/` folder** (immich) — immich has one owner; Quincy has two, and ADR-0001 rejects a shared schema folder.
- **Generating the Rust client in `build.rs`, uncommitted** (zed, atuin protos) — two rules for generated code instead of one, and code the agent can't read.
- **A Cargo workspace with `crates/` now** (zed, spacedrive) — atuin split its database crates and merged them back; zed's ~250 crates suit a large team.
- **A `spikes/` throwaway folder** (decided earlier the same day, reversed) — filtering in Go would be code undone right after, against ADR-0003.
- **justfile** — a third tool next to mise, which already runs tasks.

## Consequences

- `backend/` and `frontend/` are deleted; git keeps their history.
- Postponed until their trigger: Cargo workspace (Tauri core exists), `bridges/contract` (second Go bridge), root `e2e/` with a fake event source (app or second bridge exists), oasdiff breaking-change check (an installed app can't update in step with the server), CI path filters (CI over 5 minutes), Dockerfiles (the server leaves the Mac; each goes in its deployable's folder).
