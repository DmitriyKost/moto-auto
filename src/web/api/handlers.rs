use axum::{debug_handler, http::StatusCode, response::Redirect, Extension, Form, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    database::user::{create_user, update_user},
    models::User,
    web::session::{ApiKey, Cache, API_KEY},
};

use super::common::get_user_id;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginForm {
    login: String,
    password: String,
}

pub async fn login(
    session: Session,
    cache: Extension<Cache>,
    Form(_login): Form<LoginForm>,
) -> Result<Redirect, StatusCode> {
    let apikey = ApiKey(Uuid::new_v4().to_string());
    session.insert(API_KEY, &apikey).await.unwrap();
    cache.write().unwrap().insert(apikey.0, "1".to_string());
    Ok(Redirect::to("/admin"))
}

pub async fn admin_update_user(
    db: Extension<PgPool>,
    session: Session,
    cache: Extension<Cache>,
    Form(user): Form<User>,
) -> Result<Json<User>, StatusCode> {
    log::error!("Fuck");
    let mut new_passwordhash: Option<String> = None;
    if !user.passwordhash.is_empty() {
        new_passwordhash = Some(sha256::digest(&user.passwordhash));
    }
    let mut new_user = user.clone();
    new_user.passwordhash = new_passwordhash.clone().unwrap_or_default();
    if let Ok(Some(user_id)) = get_user_id(cache, session).await {
        if let Ok(created_user) = create_user(&db, user_id, &user).await {
            return Ok(Json(created_user));
        } else {
            if let Ok(updated_user) = update_user(
                &db,
                user_id,
                &user.username,
                new_passwordhash.as_deref(),
                Some(&user.role),
                Some(user.branch_id),
            )
            .await
            {
                return Ok(Json(updated_user));
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
