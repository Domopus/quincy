use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    /// Encrypted Spotify access token
    #[serde(skip_serializing)]
    pub spotify_access_token: String,
    /// Encrypted Spotify refresh token
    #[serde(skip_serializing)]
    pub spotify_refresh_token: String,
    pub created_at: DateTime<Utc>,
}
