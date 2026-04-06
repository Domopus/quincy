use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::service::ShareService;

#[derive(Deserialize)]
pub struct FeedQuery {
    pub recipient: Uuid,
}

/// GET /shares?recipient=<uuid>
pub async fn get_feed(
    State(pool): State<PgPool>,
    Query(q): Query<FeedQuery>,
) -> impl IntoResponse {
    let svc = ShareService::new(pool);
    match svc.feed(q.recipient).await {
        Ok(shares) => (StatusCode::OK, Json(shares)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /shares/:id/queue  — add to Spotify queue + mark listened
pub async fn queue_share(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let svc = ShareService::new(pool);
    // TODO: call spotify service to POST /me/player/queue
    match svc.mark_listened(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /shares/:id/dismiss
pub async fn dismiss_share(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let svc = ShareService::new(pool);
    match svc.mark_dismissed(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// POST /shares/queue-all?recipient=<uuid>
pub async fn queue_all(
    State(pool): State<PgPool>,
    Query(q): Query<FeedQuery>,
) -> impl IntoResponse {
    let svc = ShareService::new(pool);
    // TODO: call spotify service for each share before marking
    match svc.mark_all_listened(q.recipient).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
