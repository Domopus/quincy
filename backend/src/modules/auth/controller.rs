use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;

use super::service::AuthService;

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: Option<String>,
}

/// GET /auth/spotify — redirect to Spotify authorize URL
pub async fn spotify_login() -> impl IntoResponse {
    let client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be set");
    let redirect_uri =
        std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set");
    let scope = "user-modify-playback-state user-read-private";

    let url = format!(
        "https://accounts.spotify.com/authorize?response_type=code&client_id={client_id}&scope={scope}&redirect_uri={redirect_uri}"
    );

    Redirect::to(&url)
}

/// GET /auth/spotify/callback
pub async fn spotify_callback(
    State(pool): State<PgPool>,
    Query(params): Query<CallbackParams>,
) -> impl IntoResponse {
    let svc = AuthService::new(pool);
    let redirect_uri =
        std::env::var("SPOTIFY_REDIRECT_URI").expect("SPOTIFY_REDIRECT_URI must be set");

    match svc.spotify_callback(&params.code, &redirect_uri).await {
        Ok(_user) => Redirect::to("quincy://auth/success"),
        Err(_) => Redirect::to("quincy://auth/error"),
    }
}
