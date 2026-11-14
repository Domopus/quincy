-- Accounts, Chat connections and Conversations (ADR-0006, ADR-0007).
-- Prototype: the developer's Account is inserted by hand; signup (password, email code) comes later.

CREATE TABLE accounts (
    id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email             TEXT NOT NULL CONSTRAINT accounts_email_check CHECK (email ~ '^[^@\s]+@[^@\s]+$'),
    preferred_service TEXT NOT NULL
        CONSTRAINT accounts_preferred_service_check CHECK (preferred_service IN ('deezer', 'spotify')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Login lookup; one Account per address whatever its case.
CREATE UNIQUE INDEX accounts_lower_email_key ON accounts (lower(email));

CREATE TABLE chat_connections (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts ON DELETE CASCADE,
    source     TEXT NOT NULL CONSTRAINT chat_connections_source_check CHECK (source IN ('whatsapp')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- Foreign key; "my Chat connections"; the Feed's first join.
CREATE INDEX chat_connections_account_id_idx ON chat_connections (account_id);

CREATE TABLE conversations (
    id                 BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    chat_connection_id BIGINT NOT NULL REFERENCES chat_connections ON DELETE CASCADE,
    external_id        TEXT NOT NULL,
    kind               TEXT NOT NULL CONSTRAINT conversations_kind_check CHECK (kind IN ('direct', 'group')),
    name               TEXT,  -- NULL: the bridge hasn't sent a name yet
    -- Also serves the foreign key and the Feed's join on chat_connection_id.
    CONSTRAINT conversations_chat_connection_id_external_id_key UNIQUE (chat_connection_id, external_id)
);
