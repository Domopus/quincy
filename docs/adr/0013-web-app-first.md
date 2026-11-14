# 0013 — Installable web app first in React, Tauri later

Status: Accepted (2026-11-14). Amends `product.md` "Targets" (was "Built with Tauri").

## Context

The WhatsApp session stays on the server for the prototype (ADR-0002 "Server or phone"), so the client is a list of Shares with a button that opens a link. The external review (`docs/reviews/2026-11-10-fable-review.md`) advised "web app first, Tauri later". The goal stays one codebase for web, Android and iOS.

## Decision

The client is an installable web app (PWA) in React + TypeScript, built with Vite, in `app/`. Tauri 2 wraps the same frontend when a native app is needed (decided the same day); the web app keeps working on its own. Tauri over Capacitor: its Rust core could host the WhatsApp client if the server-or-phone review moves the session onto the phone (ADR-0002), and it covers desktop.

## Rejected

- **Capacitor as the wrapper** — more mature on mobile (official push, share and deep-link plugins, in production since 2019), and the better fit while the session stays on the server. Rejected for Tauri's Rust core and desktop support. Revisit if Tauri's mobile plugins block a needed feature.
- **Svelte or Solid** — lighter, but a smaller ecosystem; for a list of cards the performance gap won't show.
- **Tauri now** — its mobile side is the least proven piece (Tauri 2.0 did not make mobile first-class), and an APK to build, sign and reinstall slows every UI change. Nothing in the prototype needs native code.
- **Expo (React Native + react-native-web)** — real native UI and a mature mobile stack, but the web is second-class (not the DOM, constrained styling), and switching later rewrites the UI. The hard-to-reverse choice is web-first versus Expo; web wins because the prototype runs in a browser first.
- **Flutter, Compose Multiplatform** — canvas rendering on the web (Compose's web target is still Beta); Dart or Kotlin only for the UI.
- **Dioxus** — Rust everywhere and types shared with the server, but a small ecosystem, Android support described as experimental, and 0.8 in alpha.
- **GPUI (Zed)** — desktop only.

## Consequences

- Push on GrapheneOS (checked 2026-11-14, reported, not tested): web push in Vanadium works only with sandboxed Google Play in the same profile, because it goes through FCM; the developer's phone has it. Tauri's push plugins are all community-made and also need FCM. A User without Google Play needs UnifiedPush (Fennec and IronFox reportedly support it for web push). The prototype needs no push.
- `app/` starts with `src/` only; `src-tauri/` is added with Tauri, as ADR-0010 planned, and brings the Cargo workspace.
- The phone must reach the server (LAN or Tailscale), as it would with any client.
