# Progress

_Rewrite this file; don't append. Git keeps the history._

## Where we are (2026-11-22)

Spike A's week is read and spike B's Deezer part is run (results under "Next step" 1 and 2). Spike B is done: the Spotify Web API needs a Premium app owner (403), so ADR-0014 is blocked and Spotify links rely on the WhatsApp preview. Direction for the long run written in `product.md` "Direction" (Plugins); the prototype's scope is unchanged.

### Before (2026-11-14)

Re-framing the project after a 6-month break. No new code yet. Decisions are in `docs/adr/`, `GLOSSARY.md` and `product.md`. Sessions of 2026-10-25 and 2026-11-03 grilled the bridge contract, Spotify data rules and Item identity. On 2026-11-14 an external review by Claude Fable 5.2 (`docs/reviews/2026-11-10-fable-review.md`, opinion, not decisions) was checked and its doc inconsistencies fixed. Then the prototype was defined (`product.md` "Audience" and "Prototype success"): Quincy is a personal tool; the prototype's only User is the developer, whose Account is created in the database (ADR-0007); friends follow if it succeeds. A reply assistant is a project goal (`product.md` "Later"). The review's spikes-first order was adopted: measure real messages before designing further. The repo layout was settled from six reference repos (ADR-0010) and the April scaffold deleted.

Review points still worth keeping in mind: the whatsmeow session store is the asset to protect; WhatsApp-side ban risk deserves a plain warning at pairing; ADR-0003's rejection reason is stale. Corrections to the review: a search-link-only Spotify output shrinks the expiry section but doesn't remove it (Spotify `source` rows remain); "derived identity" also needs Listened and Shares rekeyed, since both use `item_id`.

## Next step

Repo layout settled (ADR-0010, 2026-11-14): spike A is written straight into `server/` and `bridges/whatsapp/`, server webhook first (it owns the contract), then the bridge. No throwaway folder.

1. **Spike A, the bridge:** whatsmeow on the developer's account, logging only music links and their previews for a week. Runs on the developer's Mac (decided 2026-11-14): home IP, no server; WhatsApp resends what arrived while the Mac slept, which also tests catch-up. Server conditions (always-on, datacenter IP) are tested later, with the server-or-phone review before friends join. Answers: preview presence for Spotify links, `musicMetadata` fill, history sync depth. The server's webhook filters music links and writes them to a temporary table `incoming_links` (decided 2026-11-14), not to `shares`/`items`, which stay undesigned until the numbers are in. One row per music link: URL, service, link kind; preview present or not, with `title`, `description`, `musicMetadata`, thumbnail size; time, `direct`/`group`, hashed sender ID, origin (live, catch-up, history sync). Never the message text or any name. A global count of messages received is the denominator. Spike B reads this table; it is dropped when the real tables land. **Done 2026-11-14:** the server's webhook (`server/src/webhook.rs`) and the WhatsApp bridge (`bridges/whatsapp/`), both tested; not yet paired with a real phone. Bridge shortcuts for the spike, each to revisit before friends join:
- No encrypted event buffer yet (ADR-0002): when the server is down the bridge withholds the receipt, but WhatsApp's resend can't be decrypted twice, so that message is lost. Keep the server running during the week.
- Conversation names are not sent (the spike's table ignores them anyway).
- The User's own messages, status updates and channels are not forwarded.
- No command API (huma) yet: pairing is a CLI flag.
- `catch_up` = messages between `OfflineSyncPreview` and `OfflineSyncCompleted`; history sync failures are logged, not retried.
**Paired 2026-11-14 by QR code; code pairing still fails.** With a code, the phone (GrapheneOS, Android-based) shows "Impossible de connecter l'appareil" although the whole exchange succeeds on the bridge's side (DEBUG logs): `companion_hello` accepted with `"Chrome (Mac OS)"`, the phone's `primary_hello` arrives, `companion_finish` accepted, then WhatsApp never sends `pair-success` and gives no reason. Failed 4+ times in a row. whatsmeow issues #1039 (first code attempt often fails) and #1199 (anti-abuse rejects repeated links) are the closest reports. The QR code paired at the first try. So the bridge pairs by QR when run without `-pair`, logs whatsmeow at DEBUG while pairing, logs every pairing event, then silences whatsmeow once pairing ends. It must keep running after `pair-success`: whatsmeow reconnects at once (stream error 515) to log in, and a bridge that stopped there was dropped by WhatsApp (`401: logged out from another device` on the next start). Code pairing is kept; why it fails is not investigated further for now. After the week (not before: re-pairing would stop it, and repeated links may trigger WhatsApp's anti-abuse), test it as a second linked device with its own store (`WHATSAPP_STORE=<other path> mise run bridge -- -pair <number>`); if it still fails, retry from a stock Android phone to tell whether GrapheneOS is the cause.
**Week started 2026-11-14 14:01.** The pairing's history sync (INITIAL_BOOTSTRAP, then RECENT) went back to 2026-10-04: 3,072 messages counted by the server, 9 music links (5 Spotify tracks, 4 Deezer short links), all with a preview, none with `musicMetadata`. The bridge's own count was higher (5,518), because it also counted the messages it skips (the User's own, status, channels); its log now shows them apart. Then: run spike B on the week's links.
**Week read 2026-11-22** (bridge still running). Messages: 3,919 (history sync 3,072, live 276, catch-up 571; the Mac sleeps often, catch-up after sleep works, no duplicate URL). Music links: 14, about 1 in 280 messages; only 5 in the week itself (2 live, 3 catch-up). 13 of 14 from groups (5 Conversations, 8 senders). Findings:
- **Deezer:** 4/4 short links, all with a usable preview ("Artist - titre - year").
- **Spotify previews are not reliable:** 7/10 usable; 1 has no preview, 2 are empty (title `open.spotify.com`, description = the URL). This contradicts the reason ADR-0008's fallback step 2 (MusicBrainz) was deferred ("all of spike A's Spotify links had a preview"); to reconsider.
- **`musicMetadata` never filled** (0/14). Thumbnail: 0/9 in history sync, 3/5 in the week.
- **The preview description follows the sender's locale** ("Titre", "Song", "Videoclipe"): never parse it on a fixed word.
- **Clock anomaly:** both links of 2026-11-18 have `sent_at` ~50 min after `received_at`. Not investigated; it matters because the Feed sorts on `sent_at`.
- **Sample too small** (14 links) for a meaningful hit rate in spike B; read it as a smoke test.
2. **Spike B, the matcher:** run spike A's real links through Deezer; measure the hit rate and how often duplicates appear. Also live-test Spotify Search with `isrc:` on client credentials (decided 2026-10-25; if it fails, Spotify output falls back to the search link). **Deezer part run 2026-11-22** on the 14 links (throwaway script, not in the repo; Spotify links queried with the preview's title and first artist only, ADR-0008):
   - **Spotify → Deezer:** 6/10 found. 3 have no usable preview (placeholder, ADR-0008 step 3). Of the 7 with a preview, 6 matched exactly on title + artist; 1 missed: Pink Floyd "Louder Than Words" is "Side 4, Pt. 4: Louder Than Words" on Deezer.
   - **Version trap without duration:** "Latinoamérica" (Calle 13) gives the studio take (`USSD11000423`, 298 s) and a live one (`FR5R00905303`, 403 s) once "(feat. …)" is cleaned off; only the full title tells them apart. So: compare the raw title first, the cleaned title only as a fallback, and several ISRCs among the hits → `low`.
   - **Deezer short links:** 4/4 resolve (`link.deezer.com/s/…` redirects to the track URL), and the API gives ISRC and duration.
   - **Same recording, two Deezer IDs, 3 of 4 times:** single and album (e.g. `CAN112501082`: 3387469161 on the album, 3358931931 on the single). `/track/isrc:` returns the single, not the shared ID; plain search sometimes returns only the other ID; duration can differ by 1 s (169/168). "Same ISRC = same Song" (ADR-0012) absorbs it; without it the Feed would show duplicates.
   - **Not done:** Spotify Search with `isrc:` (needs Spotify client credentials).
   - **Decided 2026-11-22 (ADR-0014, partly supersedes ADR-0008):** for the personal prototype, a shared Spotify link's ISRC is read from the Spotify Web API with the developer's own key, in one removable module. **Blocked the same day:** with valid client credentials (token issued), all 14 calls (`GET /tracks/{id}` on the 10 Spotify links, Search `isrc:` on the 4 Deezer ISRCs) answer 403 "Active premium subscription required for the owner of the app". The developer won't pay Premium for it, so Spotify links stay on the preview (6/10 found in spike B). This also blocks the Deezer → Spotify direction (Search `isrc:` to build a Spotify link, ADR-0008): a Spotify User would get a search link only.
3. **Item identity decided 2026-11-14:** a Song Item is one recording (ADR-0011), derived from `links` and `matches`, never merged (ADR-0012). **WhatsApp session decided 2026-11-14:** on the server for the prototype (ADR-0002 "Server or phone"); reviewed before friends join, with spike A's catch-up numbers.
4. **Schema review of ADR-0012, 2026-11-14** (global skill `postgres-schema`; candidate DDL and 8 scenarios run on Postgres 18 in `docs/reviews/2026-11-14-schema-review/`). Holds: resent messages, blocked bad Matches, matcher retry, Feed read on indexes. Applied to ADR-0012: `shares.original_url` with `link_id` NULL until a short link is resolved; Feed sorted on `sent_at`; nullable display metadata. **Decisions still to take, one at a time** (recommendation in brackets):
   1. ~~Expired Spotify Content vs Listened~~ (scenario S7). Decided 2026-11-14: deferred until a User prefers Spotify, since only such a User makes Quincy search Spotify. Then the expiry goes on the Match, never deletes a referenced link, and an API-made Spotify link stores only its ID (ADR-0012). `expires_at` removed from `schema.sql`.
   2. ~~One metadata source per link instead of one per field.~~ Decided 2026-11-14: yes, `deezer_api` or `preview`; a Deezer API read replaces a preview's fields as a group, never the reverse (ADR-0012). MusicBrainz leaves the prototype, as the external review advised: ADR-0008's fallback step 2 is deferred (low coverage; all of spike A's Spotify links had a preview).
   3. ~~`matches.method` as `text` + `check`.~~ Decided 2026-11-14: yes, with only `isrc`, `metadata`, `manual`; `confidence` stays an enum (ADR-0012, ADR-0006 table marked superseded).
   4. ~~Copy `account_id` into `conversations` and `shares`.~~ Decided 2026-11-14: no copy; the Feed joins up to `chat_connections` (ADR-0012). Scenarios S1–S6 and S8 re-run on Postgres 18 in a throwaway database: all pass; the Feed plan sorts a User's Shares in memory, fine at this scale.
   All four decided. **Migrations written 2026-11-14** from `schema.sql` (skill `postgres-schema`): `server/migrations/20261114000002_accounts_and_conversations.sql` and `…000003_catalogue.sql`, checked on a throwaway database after `incoming_links`. Additive only; the running spike server applies them at its next `mise run dev`. `accounts` gains `email` (unique on `lower(email)`, ADR-0007); `incoming_links` stays until spike B. How a webhook finds its `chat_connections` row: decided 2026-11-14 (ADR-0009: Rust-issued `connection_id`, hashed own JID unique per source, logout keeps the row, unknown id → 404 and drop); migration `…000004_chat_connection_pairing.sql`. Later: sign webhook bodies with HMAC (WAHA, GOWA do), with authorization.

## Frontend (started 2026-11-14, while spike A runs)

Server and bridge untouched until ~2026-11-21. Decided: WhatsApp session on the server for the prototype (ADR-0002); installable web app first in React, wrapped with Tauri later (ADR-0013). Next: Feed frames from Figma (skill `figma-extract-spec`; the Figma MCP server was added 2026-11-14 after a first attempt found no Figma tools) → check them against ADR-0012 and the glossary → `DESIGN.md` → Feed in `app/` on fake data following ADR-0012 (`shares` → `links` → `matches` → `listened`; sort on `sent_at`; a Share with `link_id` NULL for an unresolved short link; NULL display metadata). No Feed endpoint yet.

**Feed frames extracted 2026-11-15** (Figma section "Feed" 97:97: Normal 86:2, Empty — First Run 97:2, Inbox Zero 97:21; no Figma variables, raw values only). Aligned with the docs without a decision: "Add to queue" becomes "Open in <Preferred service>" (ADR-0004, `product.md` core loop), "Add all to queue" is dropped (`product.md`), a Listened / Not listened filter and a Listened state on the card are added (glossary), the drawn device status bar is dropped. Gaps decided one at a time:
1. **Sender name** (decided 2026-11-15): the card shows the Contact's name. The fake data carries a nullable `sender_name`; NULL shows only the Conversation name with a generic avatar (in a direct Conversation that name is the Contact anyway). To build after ~2026-11-21, when the server and bridge can change:
   - Webhook contract (ADR-0009, Rust-owned): a nullable `sender_name`, then `mise run gen`.
   - Bridge: the address-book name (`Store.Contacts.GetContact` → `FullName`, the name the User saved), else the push name (`evt.Info.PushName`; also in history-sync messages). A group sender often arrives as a LID: map it with `Store.LIDs.GetPNForLID` first, or only the push name is found. Checked in whatsmeow `ecc39170bc14`.
   - Server: a `contacts` table keyed `(chat_connection_id, sender_hash)` with a `display_name` updated on each message, not a column on `shares` (a name changes; one copy per Contact).
   - Docs: a Contact's name is new personal data; `product.md` "Privacy" and ADR-0009 say it is kept.
2. **"Dismiss" replaced by the Listened toggle** (decided 2026-11-15): the secondary button reads "Mark as listened", or "Mark as not listened" on a Listened card, so a misclick (or an open by mistake, which marks Listened) is undone in one tap. It writes or deletes the `listened` row. A Listened card stays in the Feed, dimmed, with a marker. Inbox Zero shows when the Not listened filter is empty but the Feed has Shares; Empty — First Run when the Feed has none. No "hide": it would need a table and a term for what Listened already covers.
3. **No album on the card** (decided 2026-11-15): the line under the title shows the artist only; the cover places the record. `links` has no album, and only the Deezer API gives one reliably (a Deezer preview's description is "Artist - titre - 1969"). If wanted later: a nullable `links.album`, after ~2026-11-21.
4. **Undrawn states** (decided 2026-11-15, detail in `DESIGN.md`): no Match on the Preferred service → "Search on Deezer" (title + artist), or "Open in <shared service>" with no metadata either; Listened then goes on the shared link, so a later Match turns the card Not listened (accepted, as for the Spotify expiry in ADR-0012). Unresolved short link → "Deezer link" / "Loading song details…", opens `original_url`, Listened toggle disabled (`listened` needs a `link_id`). NULL title → "Unknown song", no artist line. No cover → accent-gradient thumb with a note and a fixed hero gradient; with a cover, the hero tint is computed from it in the client.

All four decided; `DESIGN.md` written. Next: build the Feed in `app/` from `DESIGN.md`, on fake data.

## Open questions

- **History sync at pairing** (raised 2026-11-10 by the external review). Ingest the Shares from WhatsApp's history sync so the Feed is full on day one? If yes, how far back. `product.md` onboarding assumes it. Spike A gives the facts.

## Parked until the spikes (2026-11-14)

The sections below stay as written; reopen one only when a spike shows it matters.

### Item identity and merging (closed 2026-11-14)

Settled: a Song is one recording (ADR-0011), and identity is derived from `links` and `matches`, with no merges (ADR-0012). Merging is gone, so most of this section is history. Still useful: decisions 2 and 6, the source ranking findings, and edge-tree rules 1–3 and 9 (to adapt to links and matches).

**Approved by the user (merge rules, obsolete since ADR-0012 except the Spotify one):**
- Base (proposed 2026-10-25): the older Item survives; the absorbed Item's `source` rows take the match's `method`/`confidence`, so translation confidence stays honest; Shares and Listened move over (earliest Listened time kept); each merge is logged in `item_merges` for manual undo.
- 2026-11-03: the survivor's empty fields are filled from the absorbed Item (how overwrite works is still open, see source ranking).
- 2026-11-03: `low` matches become **merge candidates** for review, not merges.
- 2026-11-03: Spotify API data never fills Item fields and never feeds the matcher (makes ADR-0008 explicit; today we only keep IDs from Spotify).
- 2026-11-03, conditional: Beatles mono and its remaster are not merged (different ISRCs block an automatic merge). Confirmed by ADR-0011 (2026-11-14): they are two Items. How a `metadata` match (a Spotify link has no ISRC) avoids linking them is still open (edge-tree rule 1).

**Product decisions still to ask** (from the edge-case tree; recommendation in brackets):
1. ~~Is an Item a recording or a song in any version?~~ Decided 2026-11-14: a recording (ADR-0011).
2. ~~Exact recording missing on the User's service?~~ Decided 2026-11-22: open the Closest version, labelled "Closest version on Deezer", plus a search link. Real case from spike B: the Spotify single "Louder Than Words" (Pink Floyd) vs Deezer's album take "Side 4, Pt. 4: Louder Than Words", the same song with or without sound effects. Still to design: how a Closest version is stored, since a Match means the same Item (ADR-0012) and a Closest version is not.
3. ~~Does listening to one version mark the other as Listened?~~ No: follows from ADR-0011; Listened is on the link the User opened (ADR-0012).
4. ~~Who reviews merge candidates in the prototype?~~ Moot: no merges (ADR-0012). Was: [the developer via SQL; Users see duplicate cards meanwhile]
5. ~~Should a `high` `metadata` match merge automatically?~~ ADR-0012: a Match at `high` or above counts as the same Song; how version tags cap confidence is edge-tree rule 1. Was: [only if version tags agree and no ISRC conflicts; tightens the approved candidate rule]
6. Is region availability in scope? [one region for the prototype; store Deezer `available_countries`]

**Source ranking (open; tested live 2026-11-03 against Deezer and MusicBrainz).** Finding: one global ranking ("Deezer API > MusicBrainz > preview") is the wrong model; each source is best at a different field and every source failed somewhere.
- MusicBrainz (MB): core data CC0; ~1 req/s per IP (503 above), User-Agent required. Coverage by Deezer ISRC: FR chart tracks released 2025+: 14/23 (big singles missing months later, e.g. `FR9W12700093`, `FR59R2673925`); older chart tracks 7/7; albums released 2026-10-16..30: 9/10; niche tracks (maloya, kompa, cumbia…): 5/78. MB ISRCs are often imported from Spotify/Deezer (ISRCHunt, "Spotify ISRC submit"; possible ADR-0008 issue, unverified). Misattachments observed: recording `be4ce82b…` ("Help", 1983 compilation) holds 4 ISRCs incl. 2009 and 2015 remasters; `GBUM71505900`, `GBAYE0601465`, `QT4K42627327` each map to 2 MB recordings. MB puts remasters on one recording ("Come Together" `485bbe7f` holds `GBAYE0000944` and the 2009 `GBAYE0601690`); 2015/2019 mixes are separate.
- Per field: **ISRC** best from Deezer (label-supplied). **Duration** from the shared link's own row (same ISRC `FR59R2687617` is 201 s on Deezer 3940644351 and 198 s on 4278713452; Yo-Yo Ma BWV 1007 Deezer 146 s vs MB 151.5 s). **Title** from the shared link's service (Deezer keeps "(Remastered 2009)"; MB and Deezer's `og:title` drop it). **Artist**: Deezer `artist` gives only "Elton John" for Cold Heart 1457025872, MB gives "Elton John & Dua Lipa"; but MB writes 宇多田ヒカル where Deezer writes "Hikaru Utada", and uses U+2010 hyphens ("Yo‐Yo Ma"); Deezer `contributors` is better but needs dedupe (Billy Ray Cyrus Main+Featured on 699056192). **Cover** from the shared link's album (Deezer `/track/isrc:GBAYE0601690` returns the 1967–1970 compilation 116348128, not Abbey Road 116348452).
- Deezer preview (`og:` tags): short title, description "Artist - titre - 1969" (locale-dependent), 500px image; no duration, no ISRC.
- Clean/explicit/radio edit share a title: "Without Me" explicit 916424 (290 s), clean 73024205 (290 s), radio edit 1123203402 (260 s, no tag). Only ISRC or Deezer's explicit flag separate clean from explicit. Same ISRC `USAT21001785` is "Fuck You" (7303269) and "F**K You" (508632702).
- Deezer field search (`artist:"…" track:"…"`) returned nothing; plain `q=` worked. Odesli is not a field source (relays platforms, 10 req/min); useful as identity second opinion.
- **Recommended rule (adopted by ADR-0012, without merges):** keep display metadata on each `item_links` row with its source, not on the Item; a merge only moves rows, nothing is overwritten. Display picks per field: the User's Preferred-service row if ≥ `high` (ideally ISRC-level), else the anchoring row, else the first non-empty value. ISRCs are a set with sources; an MB-only ISRC never triggers a merge until Deezer `/track/isrc` confirms it. Disagreements are kept and flagged. A cover found by ISRC lookup stays on its row, never promoted to the Item. This would replace edge-tree rule 7 and most of the fill/overwrite questions.
- Still open from these tests: original and 2009 remaster are two Items (ADR-0011), even where MB puts them on one recording; ±3 s duration tolerance fails for classical and same-ISRC Deezer rows (try max(3 s, 3%)), and can't separate clean/explicit (previews carry neither ISRC nor flag); artist script mismatch needs MB aliases.
- Risks, worst first: the ADR-0008 MB fallback will often miss (6% niche, 61% recent chart); clean/explicit/radio edit merged when an Item is anchored only by a preview; MB ISRC misattachments causing wrong merges; preview data is built on the sender's phone and can be faked (`musicMetadata` fill unverified); artist script/typography mismatches.

**Rules that came out of the edge-case tree (2026-11-03, not yet approved; several assume Decision 1 = recording and lean on MusicBrainz). Rules 4–8 are closed by ADR-0012:**
1. Recording-level Items. Title cleaning (ADR-0005) strips version tags ("Remastered 2009", "Mono", "Live") only for the search query; a parsed `version_tag` is kept and compared when scoring. A tag on one side and none on the other caps confidence at `low`.
2. Identity rows (`source`, `isrc`, `manual`, or ≥ `high`) are unique on `(service, kind, external_id)` and used by lookup. `low` rows are pointers ("closest version"): not unique, never used by lookup.
3. Several rows per (Item, service), one `primary` (Deezer has several track IDs per ISRC: album, single, compilation). Opening order: the shared ID if it's on the User's service → primary → other available rows → closest version → search link.
4. Automatic merge only on: same ISRC, same ID, two ISRCs on one MusicBrainz recording, or `high` metadata with matching tags and no ISRC conflict. Everything else is a merge candidate (unordered pair, unique). A rejected pair is stored as `not_same`, which blocks merging and re-proposal.
5. Several ISRCs on one Item only from MusicBrainz-backed or manual merges, never from a matcher guess.
6. "Older" = smallest `created_at`, then smallest `id`. Absorbed Items keep a `merged_into` pointer; candidates, rejections and lookups follow it. A new ISRC queues a match job and never merges in the same step, so cascades end. Lock both Items per merge.
7. Fields come only from Deezer API, MusicBrainz or the preview. Each field records its source; overwrite only on a strictly better rank or a refresh of the primary row. Title/artist/duration/cover are filled as one group from one source. Covers reference their link row. A source whose row is gone drops to the lowest rank.
8. Undo: newest merge first; a field is restored only if it still holds the merge's value (`item_merges` stores old/new per field); Shares are reassigned by their `(service, external_id)`; Listened set after the merge is copied to both Items.
9. Removed or region-locked track: keep the Item and its old IDs, mark the row `gone`/`unavailable`, fall back per rule 3.

### Spotify link expiry (ADR-0006, ADR-0008)

Deferred 2026-11-14 until a User prefers Spotify; the lifecycle (expiry on the Match, re-check job, expired Match not served) is in ADR-0012. Still open for then: TTL length (Spotify gives none); what the User sees if the re-check finds a different ID or nothing (Listened stays on the old link, so the card turns Not listened).

### Working rule proposed (not yet added)

The external review advises against it: an edge-case tree with no stop makes the design grow faster than it closes. Its alternative: "an edge case earns a design rule only if it was seen in real data or the decision is hard to reverse; otherwise it goes to a Later list in one line."

Proposed to the user 2026-11-03 for `CLAUDE.md` (or global `~/.claude/rules/`): "Before proposing a design rule, run an edge-case tree: find edge cases, propose a fix for each, then find the fix's own edge cases, at least 3 levels deep. Stop a branch only when it converges, is negligible (say why), or needs a product decision. Launch agents for live API checks. Present the surviving rules and the decisions, never the first idea."

## Other open questions

- **Messenger bridge: mautrix bridgev2 connector route** (raised 2026-11-01, decide when Messenger starts). mautrix-go's bridgev2 splits network connectors (WhatsApp, Messenger via `mautrix-meta`, Signal, Telegram…) from the Matrix connector; Beeper already runs a Matrix connector with no homeserver for its local bridges. A Quincy connector could run mautrix's maintained network connectors and emit the ADR-0009 webhook, giving new sources almost for free. Check first: size of the Matrix-side interface; whether bridgev2's database stores message text (privacy promise); licence (bridge repos believed AGPL-3.0, mautrix-go and whatsmeow MPL-2.0, unverified). Keep plain whatsmeow for the WhatsApp prototype.
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
- Precedent: on 2020-10-12 Spotify made SongShift remove Spotify → rival-service transfers; about ten days later it added the "user's personal data / playlist metadata" exception, so playlist moves are allowed again (Spotify-created playlists reportedly excluded, unverified). A reversed precedent, not a standing ban.
- Migration tools (checked 2026-11-03): Deezer's "Import my music" sends Users to TuneMyMusic, which uses the Spotify Web API with the User's own login (OAuth). Since Nov 2025 Spotify's own import also runs on TuneMyMusic. This fits the exception; Quincy's core flow (a Contact's song, Quincy's app token, another User) does not. Spotify is not an EU DMA gatekeeper. Sources: support.deezer.com article 360020762638, tunemymusic.com terms, techcrunch.com 2025-11-20, appleinsider.com 2020-10-12.
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
- `DESIGN.md` created 2026-11-15 from the Feed frames: tokens, components, card and Feed states.

## Assumed (review with the user)

- _(none right now)_

## Planned backend conventions (apply when the Rust server is scaffolded, then move to `server/CLAUDE.md`)

- A shared `CrudService` trait for entity services — speeds up building new modules.
- `utoipa` for OpenAPI/Swagger docs on every controller; the generated spec is also the source for the TypeScript client (ADR-0001).

## Housekeeping done

- Removed committed `.claude/settings.local.json` (leftover personal permissions) and gitignored it.
