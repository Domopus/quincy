-- Spike A only: dropped when the real 'shares'/'items' tables land (docs/progress.md).
-- Never holds message text or names.

CREATE TYPE message_origin AS ENUM ('live', 'catch_up', 'history_sync');

CREATE TABLE incoming_links (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source               TEXT NOT NULL,
    conversation_id      TEXT NOT NULL,
    message_id           TEXT NOT NULL,
    url                  TEXT NOT NULL,
    service              TEXT NOT NULL,
    link_kind            TEXT NOT NULL,
    external_id          TEXT,
    has_preview          BOOLEAN NOT NULL,
    preview_title        TEXT,
    preview_description  TEXT,
    preview_music        JSONB,
    preview_thumb_bytes  INTEGER,
    conversation_kind    TEXT NOT NULL,
    sender_hash          TEXT NOT NULL,
    origin               message_origin NOT NULL,
    sent_at              TIMESTAMPTZ NOT NULL,
    received_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (source, conversation_id, message_id, url)
);

-- Denominator: every message the webhook receives, music or not.
-- A resent message is counted again; acceptable for the spike.
CREATE TABLE message_counts (
    day     DATE NOT NULL,
    source  TEXT NOT NULL,
    origin  message_origin NOT NULL,
    count   BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (day, source, origin)
);
