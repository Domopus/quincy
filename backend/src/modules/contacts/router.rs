use axum::{routing::get, Router};
use sqlx::PgPool;

use super::controller::get_contact;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/:id", get(get_contact))
        .with_state(pool)
}
