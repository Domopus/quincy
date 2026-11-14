# External review of the Quincy plan (2026-11-10)

**Author: Claude Fable 5 (`claude-fable-5-2`), not Opus 5.5.** Steven asked for an outside critique of the documentation written with Opus 5.5 between 2026-10-18 and 2026-11-07. I took no part in those sessions.

**Status: opinion, not decisions.** Nothing here is settled. A point becomes a decision only when Steven approves it and it lands in an ADR, the glossary or `progress.md`. Future sessions (Opus 5.5 or other): treat this file as input to discuss, and don't apply it on your own. Where I re-open a rejected alternative, I give the new argument, as `CLAUDE.md` requires.

**What I read:** `CLAUDE.md`, `GLOSSARY.md`, `docs/product.md`, `docs/progress.md`, ADR-0001 to 0009, the git log, the two scaffold manifests. **What I did not do:** no live API calls, no web checks. Claims marked _(verify)_ come from my own knowledge and may be out of date.

## Verdict

The documentation is good; the plan it describes is too big, and it is being built in the wrong order.

Four weekend sessions (Steven works on Quincy in his spare time) have produced nine ADRs and no running code yet. That is a fair amount of output for the time; my point is about where the time went. The questions that can sink the product are all empirical and still marked "unverified" in `progress.md`. Meanwhile the design effort went to Item merging, a problem a prototype with a handful of Users will barely meet. The 2026-11-03 session settled one thing (Conversation) and opened six decisions, nine draft rules, a source-ranking model and an expiry lifecycle. The design is growing faster than it is closing.

## 1. What is strong

- **The doc system.** Short ADRs with dated amendments and a Rejected section, a glossary that is actually used, `progress.md` as a resume point, "unverified" labels. A new session can pick up cold. Keep all of it.
- **The thin bridge** (ADR-0002, 0003, 0009). The bridge knows nothing about Quincy, pushes by webhook and is easy to replace. The whatsmeow delivery research (receipts, the decrypt-once catch, the encrypted buffer) is the best work in the repo.
- **Honest matching** (ADR-0005, 0006). Confidence is the weaker of the two rows, "never a silent wrong song", "searched, not found" is stored, IDs are stored instead of URLs.
- **Terms were read before building** (ADR-0004, 0008). Dropping queue and playlist writes early saved a rebuild.

## 2. What is weak in the strategy

### Order of work

- **The riskiest assumptions are untested.** ADR-0008 rests the whole Spotify → Deezer direction on WhatsApp link previews, yet `progress.md` still lists as unverified how often previews are missing and whether `musicMetadata` is filled. One day of whatsmeow on Steven's own account, logging music links for a week, answers this with real data.
- **Design depth is inverse to risk.** Merge undo with per-field restore, `not_same` pairs, `merged_into` chains and cascade control are specified. Nobody has yet measured how often two Items would need merging in a real Feed.
- **Live tests were run on the catalogue, not on the product.** The 2026-11-03 Deezer/MusicBrainz tests are careful, but they sample charts and niche genres, not what Steven's Contacts actually share.

### Scope

- **The prototype has no definition.** `product.md` does not say who uses it, how many Users, or what result would count as success. Without it, questions like "is region availability in scope?" cannot be answered, so they get designed instead.
- **Low-value parts are carried anyway.** MusicBrainz lookup by Spotify URL hit 1 of 3 in the quick test, and MB covers 6% of niche tracks by ISRC. Odesli is deprecated with no new keys (ADR-0005), yet it is a `method` value. Both can leave the prototype.
- **Spotify as an output service costs a lot.** It brings the expiry lifecycle, a re-check job, a Premium requirement on the app owner and an untested client-credentials call. ADR-0004 says the developer is on Deezer. For the prototype, a Spotify User could get the search link only, which uses no Spotify API and stores no Spotify Content. The whole "Spotify link expiry" section then disappears.

### Uneven risk handling

- **Strict with Spotify, relaxed with Meta.** ADR-0008 parses Spotify's policy clause by clause. The WhatsApp side uses an unofficial client that breaks WhatsApp's terms outright, and it gets one line ("WhatsApp can ban the User's number"). Meta is the bigger threat: it controls the only input the product has. Many sessions connecting from one datacenter IP may also raise the ban risk _(verify)_.
- **The privacy promise has a hole.** The message buffer is encrypted (ADR-0002), but `CLAUDE.md` defers "token encryption". The whatsmeow session store gives read access to every future message of every User, and it sits on the server unprotected. That is the asset to protect first.
- **Contacts never agreed.** Their messages pass through Quincy's server, even if dropped. Fine among friends; a legal question (GDPR) the day strangers sign up. No doc mentions it.
- **Audience tension.** "GrapheneOS first" targets privacy-minded people, who are the least likely to give a server a linked WhatsApp session.

## 3. Alternatives the docs have not considered

### Capturing Shares

- **The WhatsApp session on the phone, not on the server.** The linked device is the Quincy app itself (whatsmeow through gomobile, or whatsapp-rust inside the Tauri Rust core). Messages never leave the phone, so the privacy promise holds by construction. No per-User connection to host, a home IP instead of a datacenter IP, no stored session to steal. Cost: the app only syncs while it runs. Android can keep a foreground service; iOS cannot, so the Feed catches up when the app is opened, as WhatsApp queues messages for offline linked devices _(verify the queue's limits)_. ADR-0002 rejected the notification listener, which is a different thing; this option is not in the docs.
- **Forward to a Quincy number.** The User forwards a song message to a Quincy WhatsApp contact, received through the official Business Cloud API. No ban risk, works on iOS and Android, no session per User. Cost: it is manual, and a forwarded message loses the original Contact _(verify)_. Weak as the product, strong as a one-week way to test the Feed and the matcher with real Users.
- **A middle path for the prototype:** keep ADR-0002 as written, but run it for Steven's account only until the spike data is in. The server-or-phone choice is the hardest decision to reverse, so it deserves real data.

### Data model

- **Derived identity instead of merging.** Today an Item is a stored row, so two rows that turn out to be the same must be merged, and merges need undo. Alternative: the link row `(service, kind, external_id)` is the stored fact, with its own display metadata and ISRC. "Same Song" is computed: same ISRC, or a stored translation at `high` or above. Nothing is ever merged, so nothing is undone; a wrong translation is one row to delete. The "recommended rule" in `progress.md` (metadata on `item_links`, a merge only moves rows) is already halfway there. This would replace edge-tree rules 4 to 8.
- **Prototype rule for duplicates:** tolerate them. Two cards for one Song is a cosmetic flaw; a wrong merge hides a Share.

### Account and client

- **Code-only sign-in.** ADR-0007 already emails a 6-digit code. Make the code the login and drop the password: no password storage, no reset flow, one screen fewer.
- **Web app first, Tauri later.** The prototype is a list of cards with links out. A web app (installable, with web push) covers that on Android and iOS with no app-store step, and Tauri can wrap the same frontend later. Push on GrapheneOS without Play services is its own problem under Tauri _(verify plugin support)_.

## 4. Technologies

| Choice | My view |
|---|---|
| whatsmeow | Right library. Most mature, and the research backs it. |
| Postgres + sqlx, axum | Fine. Nothing to change. |
| Thin bridge + webhook | Right shape, wherever the bridge ends up running. |
| Rust server | Defensible, but ADR-0003's reasons are thin. "Logic split from the Tauri Rust core" does not apply: the logic is on the server and the Tauri core does almost nothing. A Go server would mean one language fewer. If the real reason is "Steven wants to write Rust", that is legitimate; write it in the ADR. |
| Three code generators (ADR-0001) | Too much for one webhook and three commands. `utoipa` → TypeScript for the app API is worth it. For the bridge, hand-write the five or so types on each side. Also check versions: `progenitor` and `oapi-codegen` want OpenAPI 3.0 while `utoipa` 5 and `huma` emit 3.1 _(verify)_. |
| Tauri on mobile | The least proven piece, used where it gives the least. See "web app first". |
| `CrudService` trait | Drop it. An abstraction written before the second module exists. |
| MusicBrainz, Odesli | Out of the prototype (section 2). |

## 5. How Opus 5.5 is being used

**Working well:** one decision per turn, the model recommends and the User decides, findings are written down with dates and sources, live checks instead of guesses, atomic commits. The quality of the docs shows the model is not the problem.

**Working against you:**

- **The loop has no brake.** Grilling plus an edge-case tree asks a model to find problems, and a model will always find more. Each answer creates new rules, and each rule has edge cases. The working rule proposed on 2026-11-03 ("at least 3 levels deep") would make this permanent. I advise against adopting it. A better rule: "an edge case earns a design rule only if it was seen in real data or the decision is hard to reverse; otherwise it goes to a Later list in one line."
- **The model is used where it is weakest and kept from where it is strongest.** Reading terms of service produces confident interpretations nobody can check ("likely Spotify Content", "likely doesn't apply"). Writing a small bridge and measuring real messages produces facts. Several open questions are cheaper to answer by building than by reasoning.
- **Recommendations become decisions too easily.** Most decisions are the model's bracketed recommendation plus "ok". Ask for the counter-case in the same turn: "what is the simplest thing that could work, and what do we lose?"
- **`progress.md` is three files in one.** It holds the state, an open design in progress, and a research archive, in 18 KB that every session must read first. It also pulls every session back to the merge problem. Suggested split: `progress.md` under one page; facts in `docs/research/` (streaming APIs, whatsmeow, source ranking); the merge draft in its own file.

## 6. Inconsistencies found in the docs

- ADR-0004 still says Spotify removed ISRC from track objects. ADR-0005 and `progress.md` corrected this (restored March 2026); ADR-0004 was missed.
- ADR-0008 states the Spotify preview format ("Artist · Album · Song · Year") as fact, while `progress.md` lists Spotify previews as unverified.
- ADR-0006 keeps display metadata on `items`; `progress.md` recommends moving it to `item_links`. Pending, but ADR-0006 does not flag it.
- Onboarding "ends on the Feed where the first Shares appear" (`product.md`). That only happens if the pairing history sync is ingested, which no ADR decides. It could be a strong feature (a Feed that is full on day one) and a privacy question (how far back?).
- The glossary defines **Match**; the table is `item_links`. Small, but the glossary exists to prevent this.

## 7. Order of work I would propose

1. **Define the prototype** in `product.md`: who, how many Users, what counts as success. One short session.
2. **Spike A, the bridge:** whatsmeow on Steven's account, logging only music links and their previews for a week. Answers preview presence, `musicMetadata`, history sync depth.
3. **Spike B, the matcher:** run the real links from spike A through Deezer. Measure the hit rate and how often duplicates appear.
4. **Then decide** Item identity with those numbers, and server-or-phone for the session.
5. **Park the rest:** the pending decisions 2 to 6, rules 4 to 9 and Spotify expiry move to a Later list until a spike says they matter.

On the pending Decision 1 (recording or song in any version): I agree with the recommendation, recording. It is also what the ISRC identifies, so it is the cheapest to build.

## Addendum: checks run on 2026-11-10 (Claude Fable 5)

Run after Steven read the review. These are facts, with their limits stated.

### What a Spotify track page gives a link preview

One manual fetch of `open.spotify.com/track/2EqlS6tkEnglzr7tkKAAYD` with a WhatsApp user agent (HTTP 200, same tags as with a browser user agent):

- `og:title`: "Come Together - Remastered 2009" (the version tag is kept).
- `og:description`: "The Beatles · Abbey Road (Remastered 2009) · Song · 1969". This confirms the format ADR-0008 states, so the second point of section 6 is resolved at page level.
- `og:image`: 640 px cover. Also present: `music:duration` 260, `music:release_date`, `music:musician_description`, and a `spotify:chat_preview` tag.
- **Limit:** this shows what the sender's WhatsApp can read, not what it puts in the message. A WhatsApp preview carries title, description and thumbnail; whether duration reaches `musicMetadata`, and how often the preview is missing, still needs real messages (spike A).
- This was a one-off check by hand. Quincy must not do this fetch itself (ADR-0008: scraping).

### WhatsApp terms and enforcement

- WhatsApp Terms of Service (EEA version, effective 2026-08-17), "Harm to WhatsApp or our users": users must not "directly, indirectly, through automated or other means access, use, copy…" the Services, "(a) reverse engineer…", "(f) collect information of or about our users in any impermissible or unauthorized manner", "(h) distribute or make our Services available over a network where they could be used by multiple devices". WhatsApp "may also modify, suspend, or terminate your access… anytime". A whatsmeow session breaks the automated-access clause for the User, and a hosted bridge plausibly (h) for Quincy.
- Enforcement found: account bans, temporary then permanent. WhatsApp is also reported to be testing a warning on the Linked devices screen for unofficial web clients (wabetainfo). Vendor blogs claim stricter bans on unofficial APIs since late 2025; these are low-quality sources.
- No lawsuit or cease-and-desist found against whatsmeow or Baileys or their users. The suits found target malware apps (2022).
- **Not read:** WhatsApp's help page "About unofficial apps" (it did not load). Read it before relying on this.
- **Consequence for the plan:** the risk lands on the User's own phone number, so on Steven's friends in the prototype. The pairing screen should say so in plain words, and ADR-0002 should record that Quincy breaks WhatsApp's terms by design, not only that a ban "can" happen.

Sources: whatsapp.com/legal/terms-of-service-eea; wabetainfo.com/whatsapp-is-testing-a-safety-warning-for-unofficial-web-clients; techcrunch.com/2015/03/06 (third-party client blocks); lawstreetmedia.com (2022 suits).
