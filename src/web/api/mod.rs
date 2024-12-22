use axum::{routing::post, Router};
use handlers::{admin_update_user, login, master_complete_order};

pub mod common;
mod handlers;

#[cfg(test)]
mod tests;

pub fn new_api_router() -> Router {
    let admin_router = Router::new().route("/update_user", post(admin_update_user));
    let master_router = Router::new().route("/complete_order", post(master_complete_order));
    let default_router = Router::new().route("/login", post(login));
    Router::new()
        .nest("/", default_router)
        .nest("/admin", admin_router)
        .nest("/master", master_router)
}
