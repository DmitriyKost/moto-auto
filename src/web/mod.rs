use api::new_api_router;
use axum::Router;
use front::new_front_router;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

mod api;
mod front;

pub enum WebError {
    InitError,
    ServerError,
}

pub async fn serve(addr: &str) -> Result<(), WebError> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
    let app = Router::new()
        .nest("/api/v1", new_api_router())
        .nest("", new_front_router())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| WebError::InitError)?;

    axum::serve(listener, app)
        .await
        .map_err(|_| WebError::ServerError)?;
    Ok(())
}
