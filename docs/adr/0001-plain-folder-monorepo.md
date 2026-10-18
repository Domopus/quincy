# 0001 — Plain-folder monorepo, no monorepo tool

Status: Accepted (2026-10-18)

## Context

Three languages: Go (WhatsApp bridge), Rust (server, Tauri core), TypeScript (Tauri UI). Solo developer working with an AI agent. Little shared code, but the API contracts between the three are shared.

## Decision

One repo, plain folders. Each language uses its native tooling (Cargo workspace, Go modules, pnpm). Shared schemas live in one folder and generate code per language. At most a Makefile/`mise` on top.

## Rejected

- **Separate repos** — the usual advice for small teams without shared code, but the agent loses sight of the other side of every API, and docs/ADRs would be split across repos.
- **Nx / Bazel / Moon** — built for large teams; the real lock-in is the tool, not the folders.

## Consequences

Switching later is cheap: `git filter-repo` splits a folder out with its history. Reference project with the same stack: thinkter/rust-meow.
