use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "spotify_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SpotifyType {
    Track,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Share {
    pub id: Uuid,
    pub url: String,
    pub spotify_id: String,
    pub spotify_type: SpotifyType,
    pub context_message: Option<String>,
    pub conversation_name: String,
    pub author: Uuid,
    pub recipient: Uuid,
    pub received_at: DateTime<Utc>,
    pub listened_at: Option<DateTime<Utc>>,
    pub dismissed_at: Option<DateTime<Utc>>,
}
