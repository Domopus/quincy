# Glossary

Domain terms only — no implementation details.

- **User** — a person using Quincy.
- **Contact** — a person in a chat app who sends links to a User. Not necessarily a User.
- **Share** — one event: a Contact sent a music link to a User in a conversation. Holds the original link and an optional message the Contact wrote with it.
- **Feed** — the User's list of Shares, filterable by Listened / Not listened.
- **Listened** — a Song the User has opened (marked automatically) or marked by hand. Belongs to the User + Song, not to one Share: if two Contacts shared the same Song, listening once marks both. Can be toggled back to Not listened.
- **Song** — the piece of music a Share points to, independent of streaming service. _Working name; the catalogue entity name is still open._
- **Match** — the link between a Song and its ID on one streaming service, with a confidence level.
