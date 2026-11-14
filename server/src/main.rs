use anyhow::Context;
use quincy_server::{AppState, router};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let state = AppState {
        db: PgPool::connect(&database_url)
            .await
            .context("cannot reach Postgres")?,
        bridge_secret: std::env::var("BRIDGE_SECRET").context("BRIDGE_SECRET is not set")?,
        hash_key: std::env::var("SENDER_HASH_KEY").context("SENDER_HASH_KEY is not set")?,
    };
    sqlx::migrate!()
        .run(&state.db)
        .await
        .context("migrations failed")?;

    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, router(state)).await?;
    Ok(())
}
