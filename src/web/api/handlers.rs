use axum::{http::StatusCode, Form};
use log::info;

use crate::models::User;

pub async fn index() -> &'static str {
    "Hello, web!"
}

pub async fn admin_update_user(Form(user): Form<User>) -> StatusCode{
    info!("{:?}", user);
    StatusCode::OK
}
