use api::new_handler_router;
use axum::Router;

pub mod api;

pub enum WebError {
    InitError,
    ServerError,
}

pub async fn serve(addr: &str) -> Result<(), WebError> {
    let app = Router::new().nest("/api/v1", new_handler_router());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| WebError::InitError)?;

    axum::serve(listener, app)
        .await
        .map_err(|_| WebError::ServerError)?;
    Ok(())
}
