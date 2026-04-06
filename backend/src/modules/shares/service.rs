use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::{Share, SpotifyType};

pub struct ShareService {
    pool: PgPool,
}

impl ShareService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// All unplayed, un-dismissed shares for a recipient, newest first.
    pub async fn feed(&self, recipient: Uuid) -> Result<Vec<Share>> {
        let shares = sqlx::query_as::<_, Share>(
            r#"
            SELECT * FROM shares
            WHERE recipient = $1
              AND listened_at IS NULL
              AND dismissed_at IS NULL
            ORDER BY received_at DESC
            "#,
        )
        .bind(recipient)
        .fetch_all(&self.pool)
        .await?;
        Ok(shares)
    }

    pub async fn create(
        &self,
        url: &str,
        spotify_id: &str,
        spotify_type: SpotifyType,
        context_message: Option<&str>,
        conversation_name: &str,
        author: Uuid,
        recipient: Uuid,
    ) -> Result<Share> {
        let share = sqlx::query_as::<_, Share>(
            r#"
            INSERT INTO shares
              (id, url, spotify_id, spotify_type, context_message, conversation_name,
               author, recipient, received_at)
            VALUES
              (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, now())
            RETURNING *
            "#,
        )
        .bind(url)
        .bind(spotify_id)
        .bind(spotify_type)
        .bind(context_message)
        .bind(conversation_name)
        .bind(author)
        .bind(recipient)
        .fetch_one(&self.pool)
        .await?;
        Ok(share)
    }

    pub async fn mark_listened(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE shares SET listened_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_dismissed(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE shares SET dismissed_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Mark all unplayed shares for a recipient as listened.
    pub async fn mark_all_listened(&self, recipient: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE shares SET listened_at = $1
            WHERE recipient = $2
              AND listened_at IS NULL
              AND dismissed_at IS NULL
            "#,
        )
        .bind(Utc::now())
        .bind(recipient)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
