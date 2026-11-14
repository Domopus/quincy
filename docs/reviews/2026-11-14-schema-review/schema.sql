-- Candidate catalogue schema (ADR-0012 + ADR-0006 conversations + ADR-0007 accounts), for review only.

-- Stub: only what the catalogue needs from accounts (ADR-0007 owns the rest).
create table accounts (
  id                bigint generated always as identity primary key,
  preferred_service text not null
    constraint accounts_preferred_service_check check (preferred_service in ('deezer', 'spotify'))
);

create table chat_connections (
  id         bigint generated always as identity primary key,
  account_id bigint not null references accounts on delete cascade,
  source     text not null constraint chat_connections_source_check check (source in ('whatsapp')),
  created_at timestamptz not null default now()
);
create index chat_connections_account_id_idx on chat_connections (account_id); -- fk; "my connections"

create table conversations (
  id                 bigint generated always as identity primary key,
  chat_connection_id bigint not null references chat_connections on delete cascade,
  external_id        text not null,
  kind               text not null constraint conversations_kind_check check (kind in ('direct', 'group')),
  name               text,  -- NULL: the bridge hasn't sent a name yet
  constraint conversations_chat_connection_id_external_id_key unique (chat_connection_id, external_id)
);

create type match_confidence as enum ('none', 'low', 'high', 'exact');

create table links (
  id              bigint generated always as identity primary key,
  service         text not null constraint links_service_check check (service in ('deezer', 'spotify')),
  kind            text not null constraint links_kind_check check (kind in ('track')),
  external_id     text not null constraint links_external_id_check check (external_id ~ '^[A-Za-z0-9]+$'),
  isrc            text constraint links_isrc_check check (isrc ~ '^[A-Z]{2}[A-Z0-9]{3}[0-9]{7}$'), -- NULL: not known
  title           text,     -- NULL: not known (preview missing or degraded)
  artist          text,
  duration_ms     integer constraint links_duration_ms_check check (duration_ms > 0),
  cover_url       text constraint links_cover_url_check check (cover_url ~ '^https://'),
  metadata_source text constraint links_metadata_source_check check (metadata_source in ('deezer_api', 'preview')),
  created_at      timestamptz not null default now(),
  constraint links_isrc_kind_check check (isrc is null or kind = 'track'),
  constraint links_metadata_source_title_check check ((metadata_source is null) = (title is null)),
  constraint links_service_kind_external_id_key unique (service, kind, external_id),
  constraint links_id_service_kind_key unique (id, service, kind)  -- target of the matches fks
);
create index links_isrc_idx on links (isrc) where isrc is not null; -- "same Song by ISRC"

create table matches (
  link_id         bigint not null,
  link_service    text not null,
  kind            text not null,
  service         text not null,
  matched_link_id bigint,  -- NULL: searched on service, not found
  method          text not null constraint matches_method_check
                    check (method in ('isrc', 'metadata', 'musicbrainz', 'manual')),
  confidence      match_confidence not null,
  attempted_at    timestamptz not null default now(),  -- no expires_at: Spotify expiry deferred (ADR-0012)
  constraint matches_pkey primary key (link_id, service),
  constraint matches_link_fkey foreign key (link_id, link_service, kind)
    references links (id, service, kind) on delete cascade,
  constraint matches_matched_link_fkey foreign key (matched_link_id, service, kind)
    references links (id, service, kind) on delete cascade,
  constraint matches_service_check check (service <> link_service),
  constraint matches_not_found_check check ((matched_link_id is null) = (confidence = 'none'))
);
create index matches_matched_link_id_idx on matches (matched_link_id); -- fk cascade
create index matches_retry_idx on matches (attempted_at) where matched_link_id is null; -- ~7-day retry job

create table shares (
  id              bigint generated always as identity primary key,
  conversation_id bigint not null references conversations on delete cascade,
  message_id      text not null,
  original_url    text not null,  -- as sent; dedup key, survives an unresolved short link
  link_id         bigint references links on delete restrict,  -- NULL: short link not resolved yet
  sender_hash     text not null constraint shares_sender_hash_check check (sender_hash ~ '^[0-9a-f]{32}$'),
  message         text,  -- NULL: the Contact wrote nothing else
  sent_at         timestamptz not null,
  received_at     timestamptz not null default now(),
  constraint shares_conversation_id_message_id_original_url_key unique (conversation_id, message_id, original_url)
);
create index shares_conversation_id_sent_at_id_idx on shares (conversation_id, sent_at, id); -- Feed keyset, per conversation
create index shares_link_id_idx on shares (link_id); -- fk restrict
create index shares_unresolved_idx on shares (received_at) where link_id is null; -- resolve job

create table listened (
  account_id  bigint not null references accounts on delete cascade,
  link_id     bigint not null references links on delete restrict,
  listened_at timestamptz not null default now(),
  constraint listened_pkey primary key (account_id, link_id)
);
create index listened_link_id_idx on listened (link_id); -- fk restrict
