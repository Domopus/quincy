# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is Quincy.fm

Quincy is a desktop app that aggregates songs shared by friends via chat apps (WhatsApp for the POC) into an infinite-scroll feed, so you can queue them to your streaming service (Spotify for the POC) without interrupting your current session.

### Core product loop

1. A friend sends a Spotify link in a WhatsApp conversation.
2. Quincy detects it and creates a **post** in your feed.
3. Each post shows:
   - Header: *"{friend name} has shared a song on Spotify in {conversation name}"*
   - Optionally: *"...and wrote: [message text]"* (if they wrote something alongside the link)
   - An embedded player/preview (album art, title, artist) from the streaming service
   - An **"Add to queue"** button
4. At the top of the feed, **"Add all to queue"** adds every unplayed song at once.
5. Once a song is queued, it is marked as **listened** in the DB and excluded from future "Add all" calls.
6. A share can also be **dismissed** (archived without listening).

### Extensibility

The architecture must stay clean for future expansion:

- **Chat sources:** WhatsApp (MVP) → Messenger, others later. Chat integrations are isolated modules with no DB access.
- **Streaming services:** Spotify (MVP) → Deezer, others later. Streaming integrations are isolated modules with no DB access.
- **Share types:** Songs only (MVP) → albums, artists, playlists later.

### Auth

- **Streaming service OAuth = user identity.** There is no separate username/password. Your display name in Quincy is your name on Spotify (or whichever service).
- One-time OAuth setup; token refresh is handled silently in the background — the user never re-authenticates.
- WhatsApp pairing is also one-time; it must never require the user to re-pair.
- Future: multiple streaming services connected to one account.

### Profile page

Lower-priority feature: a profile page where the user can see the songs *they* have shared.

---

## Tech Stack

- **Frontend:** React + Vite + Tauri (scaffold Tauri from day one; Tauri-specific features deferred)
- **Backend:** Rust (Axum) — single binary, single process
- **Database:** PostgreSQL
- **WhatsApp:** `whatsapp-rust` crate running as a tokio background task inside the Axum process
- **Spotify:** OAuth + Web API (`POST /me/player/queue`)

## Commands

> Commands will be added here once the project is scaffolded.

Expected structure once set up:

```bash
# Backend (Rust/Axum)
cargo build                  # build backend
cargo run                    # run backend dev server
cargo test                   # run all tests
cargo test <module>::<test>  # run a single test

# Frontend (React + Vite)
npm run dev                  # start Vite dev server
npm run build                # production build
npm run lint                 # lint

# Tauri
npm run tauri dev            # run desktop app in dev mode
npm run tauri build          # build desktop app
```

## Backend Architecture

The backend follows a NestJS-inspired module layout:

```
backend/src/
  modules/
    auth/       mod.rs, service.rs, controller.rs, router.rs
    users/      mod.rs, entity.rs, service.rs, controller.rs, router.rs
    contacts/   mod.rs, entity.rs, service.rs, controller.rs, router.rs
    shares/     mod.rs, entity.rs, service.rs, controller.rs, router.rs
    whatsapp/   mod.rs, service.rs   ← no DB access, pure service
    spotify/    mod.rs, service.rs   ← no DB access, pure service
```

Key rules:
- `whatsapp/` and `spotify/` modules are fully DB-agnostic — pure service functions, no repository access.
- `users/`, `contacts/`, and `shares/` services implement a shared `CrudService` trait.
- All controllers are documented with Swagger via `utoipa`.
- Auth is streaming-service OAuth (Spotify = identity). WhatsApp is an integration, not auth.

## WhatsApp Integration

- Uses the **pairing code flow**: user enters an 8-digit code in WhatsApp → Linked Devices.
- The WhatsApp listener runs as a tokio task inside the same binary as the Axum server — no separate bridge service.
- If `whatsapp-rust` proves too immature, the fallback is `whatsmeow` (Go) as an isolated sidecar service.

## Database Schema

### users
| column | type | notes |
|---|---|---|
| id | UUID | PK |
| display_name | String | from streaming service |
| spotify_access_token | String | encrypted |
| spotify_refresh_token | String | encrypted |
| created_at | Timestamp | |

### contacts
WhatsApp contacts who sent links — not Quincy users.

| column | type | notes |
|---|---|---|
| id | UUID | PK |
| phone_number | String | unique |
| first_name | String | nullable |
| last_name | String | nullable |

### shares
| column | type | notes |
|---|---|---|
| id | UUID | PK |
| url | String | original URL |
| spotify_id | String | |
| spotify_type | Enum | `track` (albums/playlists deferred) |
| context_message | String | nullable — message the sender wrote alongside the link |
| conversation_name | String | WhatsApp conversation/group name |
| author | UUID | FK → contacts.id |
| recipient | UUID | FK → users.id |
| received_at | Timestamp | |
| listened_at | Timestamp | nullable — null = unplayed |
| dismissed_at | Timestamp | nullable — dismissed without listening |

## Architecture Decisions (Rejected Alternatives)

- **mautrix-whatsapp** — rejected: requires a full Matrix homeserver (Synapse), too heavy.
- **Baileys (Node.js)** — rejected in favor of `whatsapp-rust` to keep the entire stack in Rust.
- **Phone number as user identity** — rejected: streaming service OAuth is the identity instead.
