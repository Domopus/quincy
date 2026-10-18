# 0001 — Plain-folder monorepo, no monorepo tool

Status: Accepted (2026-10-18)

## Context

Three languages: Go (WhatsApp bridge), Rust (server, Tauri core), TypeScript (Tauri UI). Solo developer working with an AI agent. Little shared code, but the API contracts between the three are shared.

## Decision

One repo, plain folders. Each language uses its native tooling (Cargo workspace, Go modules, pnpm). At most a Makefile/`mise` on top.

Contracts are **code-first, and the owner of each API is its source of truth**. The owner's code generates the OpenAPI spec; clients (SDKs) are generated from that spec — no hand-written types, no shared schema folder.
- App ↔ server: Rust owns it → `utoipa` spec → generated TypeScript client.
- Bridge commands: Go owns them → `huma` spec → generated Rust client.
- Bridge → server webhook: Rust owns it → `utoipa` spec → generated Go client (ADR-0002).

_(Amended 2026-10-18 — previously "shared schemas live in one folder", which contradicted the utoipa plan.)_

## Rejected

- **Separate repos** — the usual advice for small teams without shared code, but the agent loses sight of the other side of every API, and docs/ADRs would be split across repos.
- **Schema-first contracts** (hand-written OpenAPI/proto in a shared folder) — a second source of truth next to the code that implements each API.
- **Nx / Bazel / Moon** — built for large teams; the real lock-in is the tool, not the folders.

## Consequences

Switching later is cheap: `git filter-repo` splits a folder out with its history. Reference project with the same stack: thinkter/rust-meow.
