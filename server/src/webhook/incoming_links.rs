//! Data access for spike A's `incoming_links` and `message_counts` tables.

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgConnection;

use super::music_links::MusicLink;
use super::{LinkPreview, Origin};

/// Counts one received message, music or not.
pub async fn count_message(
    conn: &mut PgConnection,
    day: NaiveDate,
    source: &str,
    origin: Origin,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO message_counts (day, source, origin, count)
           VALUES ($1, $2, $3, 1)
           ON CONFLICT (day, source, origin) DO UPDATE SET count = message_counts.count + 1"#,
        day,
        source,
        origin as Origin,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub struct NewIncomingLink<'a> {
    pub source: &'a str,
    pub conversation_id: &'a str,
    pub message_id: &'a str,
    pub link: &'a MusicLink,
    pub preview: Option<&'a LinkPreview>,
    pub conversation_kind: &'a str,
    pub sender_hash: &'a str,
    pub origin: Origin,
    pub sent_at: DateTime<Utc>,
}

/// Stores one music link; a resent message is ignored.
pub async fn insert(conn: &mut PgConnection, new: NewIncomingLink<'_>) -> Result<(), sqlx::Error> {
    let preview = new.preview;
    sqlx::query!(
        r#"INSERT INTO incoming_links (
               source, conversation_id, message_id, url, service, link_kind, external_id,
               has_preview, preview_title, preview_description, preview_music, preview_thumb_bytes,
               conversation_kind, sender_hash, origin, sent_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
           ON CONFLICT (source, conversation_id, message_id, url) DO NOTHING"#,
        new.source,
        new.conversation_id,
        new.message_id,
        new.link.url,
        new.link.service,
        new.link.kind,
        new.link.external_id,
        preview.is_some(),
        preview.and_then(|p| p.title.as_deref()),
        preview.and_then(|p| p.description.as_deref()),
        preview.and_then(|p| p.music_metadata.clone()),
        preview.and_then(|p| p.thumbnail_bytes),
        new.conversation_kind,
        new.sender_hash,
        new.origin as Origin,
        new.sent_at,
    )
    .execute(conn)
    .await?;
    Ok(())
}
