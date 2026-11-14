//! Quincy server: all domain logic (ADR-0003).

pub mod webhook;

use axum::Router;
use axum::routing::post;
use sqlx::PgPool;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    /// Shared secret every bridge sends as a bearer token (ADR-0002).
    pub bridge_secret: String,
    /// Key mixed into sender hashes.
    pub hash_key: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/internal/messages", post(webhook::receive))
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Quincy server", description = "Webhook for chat bridges (ADR-0009)."),
    paths(webhook::receive),
    modifiers(&BridgeSecret)
)]
pub struct ApiDoc;

struct BridgeSecret;

impl Modify for BridgeSecret {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bridge_secret",
            SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
        );
    }
}
