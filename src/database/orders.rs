use chrono;

use crate::database::{DbError, DbPool};
use crate::models::Order;

use super::branch;

pub async fn create_Order(pool: &DbPool, order: Order) -> Result<Order, DbError> {
    sqlx::query_as!(
        Order,
        r#"
        INSERT INTO moto_auto.orders (client_id, branch_id, master_id, order_date, completion_date, total_amount, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING order_id, client_id, branch_id, master_id, order_date, completion_date, total_amount, status
        "#,
        order.client_id,
        order.branch_id,
        order.master_id,
        order.order_date,
        order.completion_date,
        order.total_amount,
        order.status
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_order(
    pool: &DbPool,
    master_id: Option<i32>,
    completion_date: Option<chrono::DateTime<chrono::Utc>>,
    status: Option<String>,
    order_id: i32
) -> Result<Order, DbError> {
    sqlx::query_as!(
        Order,
        r#"
        UPDATE moto_auto.orders
        SET
            master_id = COALESCE($1, master_id),
            completion_date = COALESCE($2, completion_date),
            status = COALESCE($3, status)
        WHERE order_id = $4
        RETURNING order_id, client_id, branch_id, master_id, order_date, completion_date, total_amount, status
        "#,
        master_id,
        completion_date,
        status,
        order_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_order(pool: &DbPool, order_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.orders
        WHERE order_id = $1
        "#,
        order_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_orders(
    pool: &DbPool,
    branch_id: Option<i32>,
    master_id: Option<i32>,
    client_id: Option<i32>,
) -> Result<Vec<Order>, DbError> {
    match (branch_id, master_id, client_id) {
        (Some(id), None, None) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE branch_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, Some(id), None) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE master_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None, Some(id)) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE client_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(master_id), None) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE branch_id = $1 AND master_id = $2
                "#,
                branch_id,
                master_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), None, Some(client_id)) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE branch_id = $1 AND client_id = $2
                "#,
                branch_id,
                client_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, Some(master_id), Some(client_id)) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE master_id = $1 AND client_id = $2
                "#,
                master_id,
                client_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(master_id), Some(client_id)) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                WHERE branch_id = $1 AND master_id = $2 AND client_id = $3
                "#,
                branch_id,
                master_id,
                client_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None, None) => {
            sqlx::query_as!(
                Order,
                r#"
                SELECT * FROM moto_auto.orders
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
