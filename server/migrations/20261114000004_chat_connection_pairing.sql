-- Which chat account a Chat connection is paired with, and whether its session is logged out (ADR-0009).

ALTER TABLE chat_connections
    -- NULL: not paired yet. Hash of the account's own ID (WhatsApp JID), with the same key as senders.
    ADD COLUMN external_hash TEXT
        CONSTRAINT chat_connections_external_hash_check CHECK (external_hash ~ '^[0-9a-f]{32}$'),
    -- NULL: session live or not paired yet. A logout keeps the row so re-pairing reuses its id.
    ADD COLUMN logged_out_at TIMESTAMPTZ;

-- One Account per chat account: a number already linked elsewhere is refused. Unpaired rows don't collide.
CREATE UNIQUE INDEX chat_connections_source_external_hash_key ON chat_connections (source, external_hash)
    WHERE external_hash IS NOT NULL;
