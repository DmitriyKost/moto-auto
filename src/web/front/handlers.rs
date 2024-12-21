use axum::http::StatusCode;
use axum::{extract::Query, Extension};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::database::orders::get_orders;
use crate::database::user::get_users;
use crate::models::Order;
use crate::web::api::common::get_user_id;
use crate::web::front::views::AdminIndex;
use crate::{models::User, web::session::Cache};

use super::views::{Login, MasterIndex, OrderEdit, UserEdit};

pub async fn login() -> Login {
    Login {}
}

pub async fn admin_index(
    db: Extension<PgPool>,
    session: Session,
    cache: Extension<Cache>,
) -> Result<AdminIndex, StatusCode> {
    if let Ok(Some(user_id)) = get_user_id(cache, session).await {
        if let Ok(users) = get_users(&db, user_id).await {
            return Ok(AdminIndex { users });
        }
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn user_edit(Query(user): Query<User>) -> UserEdit {
    UserEdit { user }
}

pub async fn master_index(
    db: Extension<PgPool>,
    session: Session,
    cache: Extension<Cache>,
) -> Result<MasterIndex, StatusCode> {
    if let Ok(Some(user_id)) = get_user_id(cache, session).await {
        if let Ok(orders) = get_orders(&db, None, Some(user_id), None).await {
            tracing::info!("{:?}", user_id);
            return Ok(MasterIndex { orders });
        }
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn order_edit(Query(order): Query<Order>) -> OrderEdit {
    let st = order.total_amount.clone().unwrap_or_default();
    OrderEdit { order }
}
