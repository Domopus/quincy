use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use super::service::ContactService;

/// GET /contacts/:id
pub async fn get_contact(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let svc = ContactService::new(pool);
    match svc.find_by_id(id).await {
        Ok(Some(contact)) => (StatusCode::OK, Json(contact)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
