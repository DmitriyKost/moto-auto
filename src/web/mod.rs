use api::new_api_router;
use axum::{Extension, Router};
use front::new_front_router;
use session::Cache;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::Level;

mod api;
mod front;
mod session;

pub enum WebError {
    InitError,
    ServerError,
}

pub async fn serve(db: PgPool, addr: &str) -> Result<(), WebError> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);
    let cache = Cache::default();

    let app = Router::new()
        .nest("/api/v1", new_api_router())
        .nest("", new_front_router())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(cache))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| WebError::InitError)?;

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|_| WebError::ServerError)?;
    Ok(())
}
