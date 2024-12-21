use axum::{routing::get, Router};
use handlers::index;

pub mod handlers;

#[cfg(test)]
mod tests;

pub fn new_api_router() -> Router {
    Router::new().route("/", get(index))
}
