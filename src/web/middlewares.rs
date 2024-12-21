use askama_axum::IntoResponse;
use axum::{
    extract::Request, middleware::Next, response::Redirect,
    Extension,
};
use tower_sessions::Session;

use super::{api::common::get_user_id, session::Cache};

pub async fn auth_middleware(
    session: Session,
    cache: Extension<Cache>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if request.uri().to_string().contains("login") {
        return next.run(request).await;
    }
    if let Ok(Some(_)) = get_user_id(cache, session).await {
        return next.run(request).await;
    } 
    Redirect::to("/login").into_response()
}
