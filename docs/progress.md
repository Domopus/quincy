# Progress

_Rewrite this file; don't append. Git keeps the history._

## Where we are (2026-11-03)

Re-framing the project after a 6-month break; grilling the open questions. No new code yet. Decisions so far are in `docs/adr/` and `GLOSSARY.md`. Grilling session of 2026-10-25 settled: no username in the prototype, onboarding order, Spotify data rules (ADR-0008), Deezer + Spotify as Preferred services, chat bridge contract and delivery rules (ADR-0009), webhook failures (ADR-0002), Item naming (ADR-0006). Session of 2026-11-03: Conversation is an entity with a kind and a name (glossary, ADR-0006, ADR-0009).

## Next step

**First: discuss the external review's order of work.** The review by Claude Fable 5.2 is in `docs/reviews/2026-11-10-fable-review.md` (opinion, not decisions). Checked against the docs on 2026-11-14: its facts hold; its section 6 inconsistencies are fixed. Settled since: Quincy is a personal tool, not a market product, and a reply assistant is a project goal (`product.md` "Audience" and "Later"). This weakens the review's launch-oriented points (token encryption, GDPR, non-commercial terms). Corrections to the review: a search-link-only Spotify output shrinks the expiry section but doesn't remove it (Spotify `source` rows remain); "derived identity" also needs Listened and Shares rekeyed, since both use `item_id`. Section 7 step 1 (define the prototype) in progress: the prototype's only User is the developer, friends follow soon if it's useful (`product.md` "Audience", 2026-11-14). The prototype creates the developer's Account directly in the database, no sign-up flow (ADR-0007 amended 2026-11-14). Success criteria set (`product.md` "Prototype success"), so step 1 is done. Next: decide whether to adopt the review's spikes-first order (spike A bridge, spike B matcher) and park the merge grilling. Only then resume the grilling below, or park it if the user adopts the spikes-first order.

Resume `/grilling` on **Item identity and merging** (below), in this order:

1. Present the **source-ranking findings** (below) to the user, starting with the proposal to move display metadata onto `item_links` rows.
2. Ask the user **Decision 1: is an Item a recording or a song in any version?** It was put to the user on 2026-11-03, no answer yet. Then decisions 2–6, one per turn.
3. Settle the **Spotify link expiry** questions (below).
4. Ask whether to add the **edge-case-tree working rule** (below) to `CLAUDE.md` or `~/.claude/rules/`.
5. Write the merging section into ADR-0006 (also fix ADR-0005's title cleaning, see rule 1), close the question, then repo layout, then the grilling wrap-up (review Assumed, confirm shared understanding).

First task when the matcher is built (decided 2026-10-25): live-test that a new development-mode Spotify app can call Search with `isrc:` using client credentials. If not, Spotify output falls back to the search link.

## Open questions (for grilling)

- **Repo layout** inside the monorepo (folder names, where generated clients live).
- **History sync at pairing** (raised 2026-11-10 by the external review). Ingest the Shares from WhatsApp's history sync so the Feed is full on day one? If yes, how far back. `product.md` onboarding assumes it.

### Item identity and merging (ADR-0006)

When a match finds that two Items are the same (e.g. an ISRC lookup returns an ID another Item already owns, breaking unique `(service, kind, external_id)`).

**Approved by the user:**
- Base (proposed 2026-10-25): the older Item survives; the absorbed Item's `source` rows take the match's `method`/`confidence`, so translation confidence stays honest; Shares and Listened move over (earliest Listened time kept); each merge is logged in `item_merges` for manual undo.
- 2026-11-03: the survivor's empty fields are filled from the absorbed Item (how overwrite works is still open, see source ranking).
- 2026-11-03: `low` matches become **merge candidates** for review, not merges.
- 2026-11-03: Spotify API data never fills Item fields and never feeds the matcher (makes ADR-0008 explicit; today we only keep IDs from Spotify).
- 2026-11-03, conditional: Beatles mono and its remaster are not merged (different ISRCs block an automatic merge). The user's "ok" hangs on Decision 1, because a Spotify link has no ISRC and a `metadata` match can still link mono to the remaster.

**Product decisions still to ask** (from the edge-case tree; recommendation in brackets):
1. Is an Item a recording or a song in any version? [recording] — 2, 3 and 5 depend on it.
2. Exact recording missing on the User's service? [open the closest version, labelled "Closest version on Deezer", plus a search link]
3. Does listening to one version mark the other as Listened? [no, Listened stays per Item]
4. Who reviews merge candidates in the prototype? [the developer via SQL; Users see duplicate cards meanwhile]
5. Should a `high` `metadata` match merge automatically? [only if version tags agree and no ISRC conflicts; tightens the approved candidate rule]
6. Is region availability in scope? [one region for the prototype; store Deezer `available_countries`]

**Source ranking (open; tested live 2026-11-03 against Deezer and MusicBrainz).** Finding: one global ranking ("Deezer API > MusicBrainz > preview") is the wrong model; each source is best at a different field and every source failed somewhere.
- MusicBrainz (MB): core data CC0; ~1 req/s per IP (503 above), User-Agent required. Coverage by Deezer ISRC: FR chart tracks released 2025+: 14/23 (big singles missing months later, e.g. `FR9W12700093`, `FR59R2673925`); older chart tracks 7/7; albums released 2026-10-16..30: 9/10; niche tracks (maloya, kompa, cumbia…): 5/78. MB ISRCs are often imported from Spotify/Deezer (ISRCHunt, "Spotify ISRC submit"; possible ADR-0008 issue, unverified). Misattachments observed: recording `be4ce82b…` ("Help", 1983 compilation) holds 4 ISRCs incl. 2009 and 2015 remasters; `GBUM71505900`, `GBAYE0601465`, `QT4K42627327` each map to 2 MB recordings. MB puts remasters on one recording ("Come Together" `485bbe7f` holds `GBAYE0000944` and the 2009 `GBAYE0601690`); 2015/2019 mixes are separate.
- Per field: **ISRC** best from Deezer (label-supplied). **Duration** from the shared link's own row (same ISRC `FR59R2687617` is 201 s on Deezer 3940644351 and 198 s on 4278713452; Yo-Yo Ma BWV 1007 Deezer 146 s vs MB 151.5 s). **Title** from the shared link's service (Deezer keeps "(Remastered 2009)"; MB and Deezer's `og:title` drop it). **Artist**: Deezer `artist` gives only "Elton John" for Cold Heart 1457025872, MB gives "Elton John & Dua Lipa"; but MB writes 宇多田ヒカル where Deezer writes "Hikaru Utada", and uses U+2010 hyphens ("Yo‐Yo Ma"); Deezer `contributors` is better but needs dedupe (Billy Ray Cyrus Main+Featured on 699056192). **Cover** from the shared link's album (Deezer `/track/isrc:GBAYE0601690` returns the 1967–1970 compilation 116348128, not Abbey Road 116348452).
- Deezer preview (`og:` tags): short title, description "Artist - titre - 1969" (locale-dependent), 500px image; no duration, no ISRC.
- Clean/explicit/radio edit share a title: "Without Me" explicit 916424 (290 s), clean 73024205 (290 s), radio edit 1123203402 (260 s, no tag). Only ISRC or Deezer's explicit flag separate clean from explicit. Same ISRC `USAT21001785` is "Fuck You" (7303269) and "F**K You" (508632702).
- Deezer field search (`artist:"…" track:"…"`) returned nothing; plain `q=` worked. Odesli is not a field source (relays platforms, 10 req/min); useful as identity second opinion.
- **Recommended rule (to put to the user):** keep display metadata on each `item_links` row with its source, not on the Item; a merge only moves rows, nothing is overwritten. Display picks per field: the User's Preferred-service row if ≥ `high` (ideally ISRC-level), else the anchoring row, else the first non-empty value. ISRCs are a set with sources; an MB-only ISRC never triggers a merge until Deezer `/track/isrc` confirms it. Disagreements are kept and flagged. A cover found by ISRC lookup stays on its row, never promoted to the Item. This would replace edge-tree rule 7 and most of the fill/overwrite questions.
- Still open from these tests: should original and 2009 remaster be one Item (MB says yes, Deezer no — ties to Decision 1); ±3 s duration tolerance fails for classical and same-ISRC Deezer rows (try max(3 s, 3%)), and can't separate clean/explicit (previews carry neither ISRC nor flag); artist script mismatch needs MB aliases.
- Risks, worst first: the ADR-0008 MB fallback will often miss (6% niche, 61% recent chart); clean/explicit/radio edit merged when an Item is anchored only by a preview; MB ISRC misattachments causing wrong merges; preview data is built on the sender's phone and can be faked (`musicMetadata` fill unverified); artist script/typography mismatches.

**Rules that came out of the edge-case tree (2026-11-03, not yet approved; several assume Decision 1 = recording and lean on MusicBrainz):**
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

Spotify's terms forbid storing Spotify Content "indefinitely", so `item_links` rows for Spotify get an expiry and a re-check. The user asked (2026-11-03) for a concrete lifecycle and whether it means a cron. Draft to discuss:
- Lifecycle: 11-03 a Deezer link is shared to a Spotify User → ISRC search on Spotify → row `(item, spotify, id, isrc, exact, attempted_at 11-03, expires_at 12-03)`. Until 12-03 the row is used as-is. Before expiry a periodic job (e.g. daily) re-checks rows that are about to expire, via the matcher queue (rate limits). If a row expires without a re-check, it is not served: re-check on open, else the search link.
- So it's a re-check job, not a cleanup that deletes rows.
- Open: TTL length (Spotify gives none); re-check only rows still in some Feed, deleting the rest?; is a Spotify `source` row (ID taken from a shared link, not from the API) Spotify Content at all?; what the User sees if the re-check finds a different ID or nothing.

### Working rule proposed (not yet added)

Proposed to the user 2026-11-03 for `CLAUDE.md` (or global `~/.claude/rules/`): "Before proposing a design rule, run an edge-case tree: find edge cases, propose a fix for each, then find the fix's own edge cases, at least 3 levels deep. Stop a branch only when it converges, is negligible (say why), or needs a product decision. Launch agents for live API checks. Present the surviving rules and the decisions, never the first idea."

### Other open questions

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
- Create `DESIGN.md` when UI work starts.

## Assumed (review with the user)

- _(none right now)_

## Planned backend conventions (apply when the Rust server is scaffolded, then move to `backend/CLAUDE.md`)

- A shared `CrudService` trait for entity services — speeds up building new modules.
- `utoipa` for OpenAPI/Swagger docs on every controller; the generated spec is also the source for the TypeScript client (ADR-0001).

## Housekeeping done

- Removed committed `.claude/settings.local.json` (leftover personal permissions) and gitignored it.
