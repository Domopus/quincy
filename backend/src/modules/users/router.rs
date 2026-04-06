use axum::{routing::get, Router};
use sqlx::PgPool;

use super::controller::get_user;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/:id", get(get_user))
        .with_state(pool)
}
