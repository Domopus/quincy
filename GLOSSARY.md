# Glossary

Domain terms only — no implementation details.

- **User** — a person using Quincy.
- **Account** — a User's sign-in identity: email + password, verified by email.
- **Chat connection** — a chat source linked to an Account through a live session (WhatsApp pairing first; Messenger, Instagram later). Shares arrive through it.
- **Preferred service** — the streaming service where a User's songs open. A choice, not a login. Never call it a "connection".
- **Contact** — a person in a chat app who sends links to a User. Not necessarily a User.
- **Share** — one event: a Contact sent a music link to a User in a Conversation. Holds the original link and an optional message the Contact wrote with it.
- **Conversation** — a chat where Shares arrive: direct (one Contact) or group (several Contacts). Has a name the Feed shows, e.g. "Eve in *Running club*". Belongs to one Chat connection.
- **Feed** — the User's list of Shares, filterable by Listened / Not listened.
- **Listened** — an Item the User has opened (marked automatically) or marked by hand. Belongs to the User + Item as it exists on their Preferred service, not to one Share: if two Contacts shared the same Song, even from different services, listening once marks both. Can be toggled back to Not listened.
- **Item** — the piece of music a Share points to, independent of streaming service. Not stored as such: two links are the same Item when a Match says so (ADR-0012). Has a kind: Song (the only kind in the prototype), later Album, Playlist, Artist. Not "Release": in the industry that means a label's product (album, single, EP), which a song or playlist isn't.
- **Song** — the Item kind for a single piece of music: one recording, the level an ISRC identifies, so a remaster, live, mono or clean/explicit version is another Song (ADR-0011). The word the User sees.
- **Match** — the statement that a link on one streaming service and a link on another are the same Item, with a confidence level.
