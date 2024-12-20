use axum::{extract, http::Response};
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub async fn index() -> &'static str {
    "Hello, web!"
}
#[derive(Deserialize)]
struct Scheudle {
    client_id: i128,
    order_id: i128,
    book_at: DateTime<Utc>,
    branch_id: i128,
    master_id: i128,
}

pub fn schedule_posts(extract::Json(schedule): extract::Json<Scheudle>) -> Response<String> {
    todo!()
}
