use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::User;

pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn upsert(
        &self,
        display_name: &str,
        spotify_access_token: &str,
        spotify_refresh_token: &str,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, display_name, spotify_access_token, spotify_refresh_token, created_at)
            VALUES (gen_random_uuid(), $1, $2, $3, now())
            ON CONFLICT (spotify_access_token) DO UPDATE
              SET display_name = EXCLUDED.display_name,
                  spotify_access_token = EXCLUDED.spotify_access_token,
                  spotify_refresh_token = EXCLUDED.spotify_refresh_token
            RETURNING *
            "#,
        )
        .bind(display_name)
        .bind(spotify_access_token)
        .bind(spotify_refresh_token)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
