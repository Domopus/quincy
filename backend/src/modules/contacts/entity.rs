use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Contact {
    pub id: Uuid,
    pub phone_number: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
