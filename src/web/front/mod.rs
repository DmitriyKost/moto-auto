use axum::{routing::get, Router};
use handlers::admin_index;

mod handlers;
mod views;

pub fn new_front_router() -> Router {
    let admin_router = Router::new().route("/", get(admin_index));
    Router::new().nest("/admin", admin_router)
}
