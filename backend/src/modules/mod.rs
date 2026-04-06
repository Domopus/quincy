use axum::Router;
use sqlx::PgPool;

pub mod auth;
pub mod contacts;
pub mod shares;
pub mod spotify;
pub mod users;
pub mod whatsapp;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .nest("/auth", auth::router(pool.clone()))
        .nest("/users", users::router(pool.clone()))
        .nest("/contacts", contacts::router(pool.clone()))
        .nest("/shares", shares::router(pool.clone()))
}
