use axum::{routing::get, Router};
use sqlx::PgPool;

use super::controller::{spotify_callback, spotify_login};

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .route("/spotify", get(spotify_login))
        .route("/spotify/callback", get(spotify_callback))
        .with_state(pool)
}
