use api::new_api_router;
use axum::Router;
use front::new_front_router;

mod api;
mod front;

pub enum WebError {
    InitError,
    ServerError,
}

pub async fn serve(addr: &str) -> Result<(), WebError> {
    let app = Router::new()
        .nest("/api/v1", new_api_router())
        .nest("", new_front_router());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| WebError::InitError)?;

    axum::serve(listener, app)
        .await
        .map_err(|_| WebError::ServerError)?;
    Ok(())
}
