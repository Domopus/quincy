# 0003 — Rust server holds all logic; Go only for the WhatsApp bridge

Status: Accepted (2026-10-18)

## Context

whatsmeow forces Go into the stack. Matching, storage and the app API could live in either language.

## Decision

All domain logic (matching, database, app API) is in the Rust server. The Go bridge stays thin: session handling and forwarding. URL filtering is in Rust (ADR-0002, amended 2026-10-18).

## Rejected

- **Go server for everything** — one fewer language at runtime, but logic split from the Tauri Rust core, and weaker error handling than Rust's.

## Consequences

Go code should stay small enough to replace if a mature Rust WhatsApp client appears (see ADR-0002).
