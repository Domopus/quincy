-- Catalogue: links, Matches, Shares, Listened (ADR-0012). No items table: a Song is derived.
-- incoming_links stays until spike B has read it (docs/progress.md).

-- Ordered: "same Song" is confidence >= 'high'.
CREATE TYPE match_confidence AS ENUM ('none', 'low', 'high', 'exact');

CREATE TABLE links (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    service         TEXT NOT NULL CONSTRAINT links_service_check CHECK (service IN ('deezer', 'spotify')),
    kind            TEXT NOT NULL CONSTRAINT links_kind_check CHECK (kind IN ('track')),
    external_id     TEXT NOT NULL CONSTRAINT links_external_id_check CHECK (external_id ~ '^[A-Za-z0-9]+$'),
    isrc            TEXT CONSTRAINT links_isrc_check CHECK (isrc ~ '^[A-Z]{2}[A-Z0-9]{3}[0-9]{7}$'), -- NULL: not known
    -- Display metadata, all from metadata_source. NULL: not known (preview missing or degraded).
    title           TEXT,
    artist          TEXT,
    duration_ms     INTEGER CONSTRAINT links_duration_ms_check CHECK (duration_ms > 0),
    cover_url       TEXT CONSTRAINT links_cover_url_check CHECK (cover_url ~ '^https://'),
    metadata_source TEXT CONSTRAINT links_metadata_source_check CHECK (metadata_source IN ('deezer_api', 'preview')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT links_isrc_kind_check CHECK (isrc IS NULL OR kind = 'track'),
    CONSTRAINT links_metadata_source_title_check CHECK ((metadata_source IS NULL) = (title IS NULL)),
    CONSTRAINT links_service_kind_external_id_key UNIQUE (service, kind, external_id),
    -- Target of the matches foreign keys, which pin each side's service and kind.
    CONSTRAINT links_id_service_kind_key UNIQUE (id, service, kind)
);
-- "Same Song by ISRC".
CREATE INDEX links_isrc_idx ON links (isrc) WHERE isrc IS NOT NULL;

-- A Match: this link's recording on that service is that link.
CREATE TABLE matches (
    link_id         BIGINT NOT NULL,
    link_service    TEXT NOT NULL,
    kind            TEXT NOT NULL,
    service         TEXT NOT NULL,
    matched_link_id BIGINT,  -- NULL: searched on service, not found
    method          TEXT NOT NULL CONSTRAINT matches_method_check CHECK (method IN ('isrc', 'metadata', 'manual')),
    confidence      match_confidence NOT NULL,
    attempted_at    TIMESTAMPTZ NOT NULL DEFAULT now(),  -- no expires_at: Spotify expiry deferred (ADR-0012)
    CONSTRAINT matches_pkey PRIMARY KEY (link_id, service),
    CONSTRAINT matches_link_fkey FOREIGN KEY (link_id, link_service, kind)
        REFERENCES links (id, service, kind) ON DELETE CASCADE,
    CONSTRAINT matches_matched_link_fkey FOREIGN KEY (matched_link_id, service, kind)
        REFERENCES links (id, service, kind) ON DELETE CASCADE,
    CONSTRAINT matches_service_check CHECK (service <> link_service),
    CONSTRAINT matches_not_found_check CHECK ((matched_link_id IS NULL) = (confidence = 'none'))
);
-- Foreign key cascade.
CREATE INDEX matches_matched_link_id_idx ON matches (matched_link_id);
-- The ~7-day retry of "not found".
CREATE INDEX matches_retry_idx ON matches (attempted_at) WHERE matched_link_id IS NULL;

CREATE TABLE shares (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    conversation_id BIGINT NOT NULL REFERENCES conversations ON DELETE CASCADE,
    message_id      TEXT NOT NULL,
    original_url    TEXT NOT NULL,  -- as sent; dedup key, survives an unresolved short link
    link_id         BIGINT REFERENCES links ON DELETE RESTRICT,  -- NULL: short link not resolved yet
    sender_hash     TEXT NOT NULL CONSTRAINT shares_sender_hash_check CHECK (sender_hash ~ '^[0-9a-f]{32}$'),
    message         TEXT,  -- NULL: the Contact wrote nothing else
    sent_at         TIMESTAMPTZ NOT NULL,
    received_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- A resent message is dropped; a message with two links gives two Shares.
    CONSTRAINT shares_conversation_id_message_id_original_url_key UNIQUE (conversation_id, message_id, original_url)
);
-- Feed keyset, per conversation (no account_id copy, ADR-0012).
CREATE INDEX shares_conversation_id_sent_at_id_idx ON shares (conversation_id, sent_at, id);
-- Foreign key restrict.
CREATE INDEX shares_link_id_idx ON shares (link_id);
-- The short-link resolve job.
CREATE INDEX shares_unresolved_idx ON shares (received_at) WHERE link_id IS NULL;

CREATE TABLE listened (
    account_id  BIGINT NOT NULL REFERENCES accounts ON DELETE CASCADE,
    link_id     BIGINT NOT NULL REFERENCES links ON DELETE RESTRICT,  -- the link opened on the Preferred service
    listened_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT listened_pkey PRIMARY KEY (account_id, link_id)
);
-- Foreign key restrict.
CREATE INDEX listened_link_id_idx ON listened (link_id);
