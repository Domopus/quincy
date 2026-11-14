//! `POST /internal/messages`: every chat bridge sends every message here (ADR-0002, ADR-0009).
//! Music links are kept in `incoming_links` (spike A); everything else is dropped.

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use utoipa::ToSchema;

mod incoming_links;
pub mod music_links;

use music_links::MusicLink;

use crate::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct IncomingMessage {
    /// Chat source, e.g. `whatsapp`.
    pub source: String,
    /// Stays the same if the message is sent again.
    pub message_id: String,
    /// The Chat connection the message arrived on.
    pub connection_id: String,
    /// Sender ID on the chat platform. Hashed before storage.
    pub sender: String,
    pub conversation: Conversation,
    pub text: Option<String>,
    pub link_preview: Option<LinkPreview>,
    pub sent_at: DateTime<Utc>,
    pub origin: Origin,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Conversation {
    pub id: String,
    pub kind: ConversationKind,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConversationKind {
    Direct,
    Group,
}

#[derive(Debug, Clone, Copy, Deserialize, ToSchema, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "message_origin", rename_all = "snake_case")]
pub enum Origin {
    Live,
    CatchUp,
    HistorySync,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LinkPreview {
    /// The URL the preview was built for.
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    /// Size of the preview thumbnail; the image itself is never sent.
    pub thumbnail_bytes: Option<i32>,
    /// The platform's structured music fields, as received.
    #[schema(value_type = Option<Object>)]
    pub music_metadata: Option<serde_json::Value>,
}

#[utoipa::path(
    post,
    path = "/internal/messages",
    request_body = IncomingMessage,
    security(("bridge_secret" = [])),
    responses(
        (status = 204, description = "Accepted; the bridge may confirm receipt"),
        (status = 401, description = "Missing or wrong bridge secret"),
        (status = 500, description = "Not stored; the bridge must not confirm receipt"),
    )
)]
pub async fn receive(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(message): Json<IncomingMessage>,
) -> StatusCode {
    let expected = format!("Bearer {}", state.bridge_secret);
    if headers.get("authorization").and_then(|v| v.to_str().ok()) != Some(expected.as_str()) {
        return StatusCode::UNAUTHORIZED;
    }
    match store(&state, &message).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(error) => {
            tracing::error!(%error, source = %message.source, "message not stored");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn store(state: &AppState, message: &IncomingMessage) -> Result<(), sqlx::Error> {
    let mut links = message
        .text
        .as_deref()
        .map(music_links::find)
        .unwrap_or_default();
    let preview_url = message.link_preview.as_ref().and_then(|p| p.url.as_deref());
    if let Some(link) = preview_url.and_then(music_links::parse)
        && !links.iter().any(|l| l.url == link.url)
    {
        links.push(link);
    }

    let sender_hash = hash_sender(&state.hash_key, &message.source, &message.sender);
    let conversation_kind = match message.conversation.kind {
        ConversationKind::Direct => "direct",
        ConversationKind::Group => "group",
    };

    let mut tx = state.db.begin().await?;
    incoming_links::count_message(
        &mut tx,
        message.sent_at.date_naive(),
        &message.source,
        message.origin,
    )
    .await?;
    for link in &links {
        incoming_links::insert(
            &mut tx,
            incoming_links::NewIncomingLink {
                source: &message.source,
                conversation_id: &message.conversation.id,
                message_id: &message.message_id,
                link,
                preview: preview_for(message.link_preview.as_ref(), link, links.len()),
                conversation_kind,
                sender_hash: &sender_hash,
                origin: message.origin,
                sent_at: message.sent_at,
            },
        )
        .await?;
    }
    tx.commit().await
}

/// A message carries one preview; it belongs to the link it was built for,
/// or to the only link when the preview doesn't say.
fn preview_for<'a>(
    preview: Option<&'a LinkPreview>,
    link: &MusicLink,
    link_count: usize,
) -> Option<&'a LinkPreview> {
    preview.filter(|p| match p.url.as_deref() {
        Some(url) => url == link.url,
        None => link_count == 1,
    })
}

/// Lets the spike count repeat senders without storing who they are.
fn hash_sender(key: &str, source: &str, sender: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update([0]);
    hasher.update(source.as_bytes());
    hasher.update([0]);
    hasher.update(sender.as_bytes());
    hex::encode(&hasher.finalize()[..16])
}
