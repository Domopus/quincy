//! Webhook against a real Postgres. Fixtures hold music links only, never real message text.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use quincy_server::{AppState, router};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

const SECRET: &str = "test-secret";

fn app(db: PgPool) -> axum::Router {
    router(AppState {
        db,
        bridge_secret: SECRET.into(),
        hash_key: "test-key".into(),
    })
}

fn message(text: &str, preview: Value) -> Value {
    json!({
        "source": "whatsapp",
        "message_id": "MSG1",
        "connection_id": "conn-1",
        "sender": "33600000000@s.whatsapp.net",
        "conversation": { "id": "chat-1", "kind": "group", "name": "Running club" },
        "text": text,
        "link_preview": preview,
        "sent_at": "2026-11-14T10:00:00Z",
        "origin": "live"
    })
}

async fn post(app: axum::Router, body: &Value, secret: &str) -> StatusCode {
    let request = Request::post("/internal/messages")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {secret}"))
        .body(Body::from(body.to_string()))
        .unwrap();
    app.oneshot(request).await.unwrap().status()
}

#[sqlx::test]
async fn rejects_a_wrong_secret(db: PgPool) {
    let status = post(app(db), &message("hi", Value::Null), "nope").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn keeps_the_music_link_and_its_preview_but_not_the_text(db: PgPool) {
    let preview = json!({
        "url": "https://www.deezer.com/fr/track/916424",
        "title": "Without Me",
        "description": "Eminem - Without Me - 2002",
        "thumbnail_bytes": 4096,
        "music_metadata": { "author": "Eminem" }
    });
    let body = message(
        "SECRET-WORDS https://www.deezer.com/fr/track/916424",
        preview,
    );
    assert_eq!(
        post(app(db.clone()), &body, SECRET).await,
        StatusCode::NO_CONTENT
    );

    let row: (String, String, Option<String>, bool, Option<String>, String) = sqlx::query_as(
        "SELECT service, link_kind, external_id, has_preview, preview_title, conversation_kind FROM incoming_links",
    )
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(
        row,
        (
            "deezer".into(),
            "track".into(),
            Some("916424".into()),
            true,
            Some("Without Me".into()),
            "group".into()
        )
    );

    let leaked: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM incoming_links WHERE row_to_json(incoming_links)::text LIKE '%SECRET-WORDS%' OR row_to_json(incoming_links)::text LIKE '%33600000000%' OR row_to_json(incoming_links)::text LIKE '%Running club%')",
    )
    .fetch_one(&db)
    .await
    .unwrap();
    assert!(
        !leaked,
        "message text, sender or conversation name was stored"
    );
}

#[sqlx::test]
async fn counts_a_message_without_music_and_stores_nothing(db: PgPool) {
    assert_eq!(
        post(
            app(db.clone()),
            &message("on se voit demain ?", Value::Null),
            SECRET
        )
        .await,
        StatusCode::NO_CONTENT
    );

    let links: i64 = sqlx::query_scalar("SELECT count(*) FROM incoming_links")
        .fetch_one(&db)
        .await
        .unwrap();
    let counted: i64 = sqlx::query_scalar("SELECT sum(count)::bigint FROM message_counts")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!((links, counted), (0, 1));
}

#[sqlx::test]
async fn ignores_a_resent_message(db: PgPool) {
    let body = message(
        "https://open.spotify.com/track/2EqlS6tkEnglzr7tkKAAYD",
        Value::Null,
    );
    post(app(db.clone()), &body, SECRET).await;
    assert_eq!(
        post(app(db.clone()), &body, SECRET).await,
        StatusCode::NO_CONTENT
    );

    let links: i64 = sqlx::query_scalar("SELECT count(*) FROM incoming_links")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(links, 1);
}
