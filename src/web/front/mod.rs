use axum::{routing::get, Router};
use handlers::{admin_index, login, master_index, order_edit, user_edit};

mod handlers;
mod views;

pub fn new_front_router() -> Router {
    let view_router = Router::new()
        .route("/user_edit", get(user_edit))
        .route("/order_edit", get(order_edit));

    let admin_router = Router::new()
        .route("/", get(admin_index));

    let master_router = Router::new()
        .route("/", get(master_index));

    let default_router = Router::new() 
        .route("/login", get(login));

    Router::new()
        .nest("/admin", admin_router)
        .nest("/master", master_router)
        .nest("/views/", view_router)
        .nest("/", default_router)
}
