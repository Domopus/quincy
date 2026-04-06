use axum::Router;
use sqlx::PgPool;

pub mod controller;
pub mod router;
pub mod service;

pub use router::routes;

pub fn router(pool: PgPool) -> Router {
    routes(pool)
}
