use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::users::entity::User;

pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Exchange a Spotify OAuth authorization code for tokens and upsert the user.
    pub async fn spotify_callback(
        &self,
        code: &str,
        _redirect_uri: &str,
    ) -> Result<User> {
        // TODO: exchange code with Spotify token endpoint
        // TODO: fetch /me from Spotify to get display_name
        // TODO: upsert user in DB
        let _ = code;
        todo!("Spotify OAuth callback not yet implemented")
    }

    pub async fn refresh_token(&self, user_id: Uuid) -> Result<()> {
        // TODO: use refresh token to get a new access token and update DB
        let _ = user_id;
        todo!("Token refresh not yet implemented")
    }
}
