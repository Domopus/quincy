use axum::Router;
use sqlx::PgPool;

pub mod controller;
pub mod entity;
pub mod router;
pub mod service;

pub fn router(pool: PgPool) -> Router {
    router::routes(pool)
}
