\set ON_ERROR_STOP 0
\set ON_ERROR_ROLLBACK on
\set VERBOSITY terse
begin;
create schema review;
set search_path = review;
-- Copy schema.sql into the container first: docker compose cp docs/reviews/2026-11-14-schema-review/schema.sql postgres:/review/schema.sql
\i /review/schema.sql

\echo '== Pass 1: fk without a leading index'
select c.conrelid::regclass as table_name, c.conname as fk
from pg_constraint c
where c.contype = 'f' and c.connamespace = 'review'::regnamespace
  and not exists (select 1 from pg_index i where i.indrelid = c.conrelid
    and (string_to_array(i.indkey::text, ' ')::int2[])[1] = c.conkey[1]);
\echo '== Pass 1: nullable column in a unique constraint'
select c.conrelid::regclass, c.conname, a.attname
from pg_constraint c
join pg_attribute a on a.attrelid = c.conrelid and a.attnum = any (c.conkey)
join pg_index i on i.indexrelid = c.conindid
where c.contype = 'u' and c.connamespace = 'review'::regnamespace
  and not a.attnotnull and not i.indnullsnotdistinct;

-- Setup: the developer (Deezer), one WhatsApp connection, one group.
insert into accounts (preferred_service) values ('deezer');
insert into chat_connections (account_id, source) values (1, 'whatsapp');
insert into conversations (chat_connection_id, external_id, kind)
values (1, '120363000000000000@g.us', 'group');

\echo '== S1 Spotify share arrives live, then again in catch-up'
insert into links (service, kind, external_id, title, artist, metadata_source)
values ('spotify', 'track', '7A4n3q3BdviQHiq9AOGD7i', 'Feel The Cold', 'Foy Vance', 'preview')
on conflict (service, kind, external_id) do update set external_id = excluded.external_id returning id;
insert into shares (conversation_id, message_id, original_url, link_id, sender_hash, sent_at)
values (1, 'MSG1', 'https://open.spotify.com/track/7A4n3q3BdviQHiq9AOGD7i?si=x', (select id from links where service='spotify' and external_id='7A4n3q3BdviQHiq9AOGD7i'), repeat('a', 32), '2026-10-31 12:00+00')
on conflict do nothing;
insert into links (service, kind, external_id, title, artist, metadata_source)
values ('spotify', 'track', '7A4n3q3BdviQHiq9AOGD7i', 'Feel The Cold', 'Foy Vance', 'preview')
on conflict (service, kind, external_id) do update set external_id = excluded.external_id returning id;
insert into shares (conversation_id, message_id, original_url, link_id, sender_hash, sent_at)
values (1, 'MSG1', 'https://open.spotify.com/track/7A4n3q3BdviQHiq9AOGD7i?si=x', (select id from links where service='spotify' and external_id='7A4n3q3BdviQHiq9AOGD7i'), repeat('a', 32), '2026-10-31 12:00+00')
on conflict do nothing;
select count(*) as shares_after_resend from shares;

\echo '== S2 Deezer short link: share stored before resolution, resolved later'
insert into shares (conversation_id, message_id, original_url, sender_hash, sent_at)
values (1, 'MSG2', 'https://link.deezer.com/s/34KhGNcQjFqeMoUDfvvWj', repeat('b', 32), '2026-10-22 09:00+00');
insert into links (service, kind, external_id, isrc, title, artist, duration_ms, metadata_source)
values ('deezer', 'track', '3135556', 'GBUM71029604', 'Missing You', 'LEISURE', 201000, 'deezer_api') returning id;
update shares set link_id = (select id from links where service='deezer' and external_id='3135556') where message_id = 'MSG2';
select id, link_id is not null as resolved from shares order by id;

\echo '== S3 degraded Spotify preview (title = domain): stored with unknown metadata'
insert into links (service, kind, external_id) values ('spotify', 'track', '0chxeKv3ZAV4vnl3OVBXMg') returning id;
\echo '   ... and a title without a source is rejected:'
insert into links (service, kind, external_id, title) values ('spotify', 'track', 'abc', 'open.spotify.com');

\echo '== S4 matcher: Spotify link 1 -> Deezer, not found, then found on retry'
insert into matches (link_id, link_service, kind, service, method, confidence)
values ((select id from links where service='spotify' and external_id='7A4n3q3BdviQHiq9AOGD7i'), 'spotify', 'track', 'deezer', 'metadata', 'none');
insert into links (service, kind, external_id, title, artist, metadata_source)
values ('deezer', 'track', '2900000001', 'Feel The Cold', 'Foy Vance', 'deezer_api') returning id;
insert into matches (link_id, link_service, kind, service, matched_link_id, method, confidence)
values ((select id from links where service='spotify' and external_id='7A4n3q3BdviQHiq9AOGD7i'), 'spotify', 'track', 'deezer', (select id from links where service='deezer' and external_id='2900000001'), 'metadata', 'high')
on conflict (link_id, service) do update
set matched_link_id = excluded.matched_link_id, method = excluded.method,
    confidence = excluded.confidence, attempted_at = excluded.attempted_at
where matches.method <> 'manual';
select link_id, service, matched_link_id, confidence from matches;

\echo '== S5 bad states the constraints must block'
insert into matches (link_id, link_service, kind, service, matched_link_id, method, confidence)
values ((select id from links where service='deezer' and external_id='3135556'), 'deezer', 'track', 'spotify', (select id from links where service='deezer' and external_id='2900000001'), 'isrc', 'exact');  -- target is on deezer, not spotify
insert into matches (link_id, link_service, kind, service, matched_link_id, method, confidence)
values ((select id from links where service='deezer' and external_id='3135556'), 'deezer', 'track', 'spotify', null, 'isrc', 'high'); -- not found but high
insert into matches (link_id, link_service, kind, service, method, confidence)
values ((select id from links where service='deezer' and external_id='3135556'), 'spotify', 'track', 'deezer', 'isrc', 'none');      -- link_service lies about link 2
insert into links (service, kind, external_id) values ('deezer', 'track', 'https://deezer.com/track/1');

\echo '== S6 Feed read: page 1, with the link to open on the preferred service and Listened'
insert into listened (account_id, link_id) values (1, (select id from links where service='deezer' and external_id='2900000001'));
set enable_seqscan = off;
explain (costs off)
select s.id, s.sent_at, coalesce(m.matched_link_id, case when l.service = a.preferred_service then l.id end) as open_link,
       exists (select 1 from listened li where li.account_id = a.id
               and li.link_id = coalesce(m.matched_link_id, l.id)) as listened
from shares s
join conversations c on c.id = s.conversation_id
join chat_connections cc on cc.id = c.chat_connection_id
join accounts a on a.id = cc.account_id
left join links l on l.id = s.link_id
left join matches m on m.link_id = l.id and m.service = a.preferred_service
where cc.account_id = 1 and (s.sent_at, s.id) < ('infinity', 0)
order by s.sent_at desc, s.id desc limit 20;

-- S7 (expired Spotify Content blocked by listened) removed 2026-11-14: Spotify expiry deferred, it moves to the Match (ADR-0012).

\echo '== S8 the Deezer short link can never be resolved (dead link)'
select count(*) as unresolved_shares from shares where link_id is null;

rollback;
