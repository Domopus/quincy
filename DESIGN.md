# Design

Source: Figma file `xbD2AKVjev41Xr7IjNoe3s`, section "Feed" (97:97): Normal 86:2, Empty — First Run 97:2, Inbox Zero 97:21, extracted 2026-11-15. The file has no Figma variables: the token names below are ours, the values are Figma's. Where this file and Figma disagree, this file wins (gaps decided in `docs/progress.md` "Frontend"). Terms follow `GLOSSARY.md`.

Mobile first, designed at 390×844. Dark theme only for the prototype.

## Tokens

### Colour

| Token | Value | Use |
|---|---|---|
| `bg-app` | `#08061a` | screen background |
| `surface-bar` | `#1a1640` | top bar, bottom nav |
| `border-bar` | `#2d2660` | 1px border of the bars |
| `surface-card` | `rgba(20,14,50,0.82)` | card body |
| `text-primary` | `#ede9ff` | titles, wordmark |
| `text-secondary` | `#a89ec4` | sender name, empty-state body |
| `text-muted` | `#635d80` | Conversation name, message, ghost button, inactive tab |
| `accent-start` | `#9b6dff` | gradient start, active tab, active filter |
| `accent-end` | `#c084fc` | gradient end |
| `on-accent` | `#ffffff` | text on the accent gradient |
| `ghost-fill` | `rgba(255,255,255,0.05)` | ghost button |
| `ghost-border` | `rgba(255,255,255,0.08)` | ghost button border, inset top highlights |

Derived: `text-primary` at 70% (artist line) and 40% (time); `accent-start` at 15% (chip fill) and 40% (chip border); `bg-app` at 55% (hero scrim).

Gradients:
- `gradient-accent`: `accent-start` → `accent-end`, left to right (wordmark "Q", primary button); top to bottom on the avatar; −45° with stops 14.6% / 85.4% on the empty-state icon.
- `gradient-hero`: three stops top to bottom, `<tint>` → `<tint darker>` at 50% → `#0f0a32`. The tint comes from the cover, computed in the client. Fallback (no cover, or not computed yet): `#501ea0` → `#281478` → `#0f0a32`.
- Inbox Zero background: a radial glow of `accent-start` at 14% fading to 0, over `bg-app`.

### Type

Fonts: Syne ExtraBold (800) for display, DM Sans (optical size 14) for everything else. Line height "normal".

| Token | Font | Size | Use |
|---|---|---|---|
| `display-lg` | Syne 800 | 24 | "All caught up." |
| `display` | Syne 800 | 22 | wordmark, "Nothing here yet" |
| `title` | DM Sans 500 | 14 | song title |
| `body-light` | DM Sans 300 | 14 | empty-state body |
| `label` | DM Sans 500 | 12 | filter chips |
| `meta` | DM Sans 400 | 12 | artist line |
| `label-sm` | DM Sans 500 | 11 | sender name, buttons |
| `meta-sm` | DM Sans 400 | 11 | Conversation name, ghost button |
| `time` | DM Sans 300 | 11 | relative time |
| `message` | DM Sans 300 italic | 11 | the Contact's message |
| `micro` | DM Sans 500 | 9 | avatar initial, tab labels |

### Radius, spacing, shadow

- Radius: `full` 999 (chips, avatar, empty icon), `card` 16, `thumb` 10, `button` 8, `tab-square` 2.
- Spacing: screen padding 16; bar padding 20; card gap 14; card padding 14; hero row gap 12; song-info gap 2; body gap 8; sender row gap 6; button gap 8; icon-to-label 6; button padding 14×7 (primary), 12×7 (ghost).
- Shadow: card `0 2px 3px rgba(0,0,0,.2), 0 8px 16px rgba(0,0,0,.5)`; thumb `0 4px 16px rgba(0,0,0,.6)`; primary button `0 4px 12px rgba(155,109,255,.35)`; inset top highlight `inset 0 1px 0 ghost-border` on hero and body (`.1` alpha on the thumb).

## Layout

- **Top bar** (56): wordmark left. No "Add all to queue" (out of the prototype, `product.md`).
- **Filter row**, under the top bar: three chips, "Not listened" (default), "Listened", "All". Active chip: `accent-start` 15% fill, 40% border, `accent-start` text; inactive: `ghost-fill`, `ghost-border`, `text-muted`. Not in Figma; added because the glossary's Feed is filterable.
- **Feed**: cards in one column, newest `sent_at` first.
- **Bottom nav** (54): Feed (2×2 squares icon) and Profile (person icon). Active tab: icon and label both `accent-start` (Figma's Empty and Inbox Zero frames colour the icon `text-muted`: a slip). Profile is a placeholder in the prototype.
- No drawn status bar: the device draws its own.

## Components

### FeedCard

358 wide, radius `card`, clipped, card shadow. Two parts:

- **Hero** (86): `gradient-hero` under a blurred copy of the cover (blur 16, `bg-app` 55% scrim). Row: thumb 56×56 (`thumb` radius, cover, object-cover) + song info (gap 2, one line each, ellipsis):
  - title (`title`);
  - artist (`meta`, `text-primary` 70%). No album (decided 2026-11-15);
  - relative time of `sent_at` (`time`, `text-primary` 40%): "Just now", "5m ago", "2h ago", "Yesterday", then "12 Nov".
- **Body** (`surface-card`, padding 10/14/14/14, gap 8):
  - sender row: avatar 20 (`gradient-accent` vertical, initial in `micro`, `on-accent`) + Contact name (`label-sm`, `text-secondary`) + "· <Conversation name>" (`meta-sm`, `text-muted`);
  - the Contact's message, if any, in quotes (`message`, `text-muted`, wraps);
  - actions: primary button + ghost button.

**Buttons**
- Primary: `gradient-accent`, `on-accent` text, play icon, primary shadow. Label depends on the card state (below).
- Ghost: the Listened toggle, "Mark as listened" / "Mark as not listened". Replaces Figma's "Dismiss" (decided 2026-11-15).

### EmptyState

Centred, gap 16: icon circle (64, `gradient-accent` −45°, white music note 32), title, body (`body-light`, `text-secondary`, 300 wide, centred).

## Card states

The card reads a Share, its link, that link's Match for the Preferred service, and Listened (ADR-0012).

| State | When | Hero | Primary button | Listened toggle |
|---|---|---|---|---|
| Normal | Match ≥ `high`, or the shared link is on the Preferred service | cover, title, artist | "Open in Deezer": opens the matched link, marks Listened | on the opened link |
| Listened | a `listened` row on that link | as Normal; whole card at 60% opacity; "✓ Listened" after the time | "Open in Deezer" | "Mark as not listened" |
| No Match | Match `none`, `low`, or not attempted yet | as Normal | "Search on Deezer": search with title + artist | on the shared link (a later Match turns the card Not listened) |
| No Match, no metadata | as above, and `title` NULL | as Unknown | "Open in <shared service>": opens `original_url` | on the shared link |
| Unresolved short link | `link_id` NULL | placeholder thumb; title "Deezer link"; artist line "Loading song details…" | "Open": opens `original_url` | disabled (`listened` needs a `link_id`) |
| Unknown | `title` NULL | title "Unknown song"; no artist line | per Match | per Match |
| No cover | `cover_url` NULL | placeholder thumb (`gradient-accent`, music note), no blur, fallback `gradient-hero` | per Match | per Match |

**Sender:** the Contact's name (decided 2026-11-15; the server stores it after ~2026-11-21). With no name: the sender row shows only the Conversation name, avatar with a music note instead of an initial. A direct Conversation shows the Contact's name alone (no "· <Conversation name>", the two are the same). A Conversation with no name shows "WhatsApp group" or "WhatsApp chat".

## Feed states

| State | When | Content |
|---|---|---|
| List | the filter has Shares | cards |
| Empty — First Run | the Feed has no Share at all | "Nothing here yet" / "When friends share songs with you on WhatsApp, they'll show up here." Filter row hidden. |
| Inbox Zero | filter "Not listened" is empty, the Feed has Shares | Inbox Zero glow, "All caught up." / "You've listened to everything. Check back when friends share more." |
| Filter empty | filter "Listened" is empty | EmptyState: "Nothing listened yet" / "Songs you open show up here." |
