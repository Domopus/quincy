use axum::{routing::{get, post}, Router};
use sqlx::PgPool;

use super::controller::{dismiss_share, get_feed, queue_all, queue_share};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(get_feed))
        .route("/queue-all", post(queue_all))
        .route("/:id/queue", post(queue_share))
        .route("/:id/dismiss", post(dismiss_share))
        .with_state(pool)
}
