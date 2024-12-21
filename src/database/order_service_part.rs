use bigdecimal::BigDecimal;

use crate::database::{DbError, DbPool};
use crate::models::OrderServicePart;

use super::order_service;

pub async fn create_order_service_part(
    pool: &DbPool,
    order_service_part: OrderServicePart,
) -> Result<OrderServicePart, DbError> {
    sqlx::query_as!(
        OrderServicePart,
        r#"
        INSERT INTO moto_auto.order_service_part (part_id, order_service_id, quantity)
        VALUES ($1, $2, $3)
        RETURNING order_service_part_id, part_id, order_service_id, quantity
        "#,
        order_service_part.part_id,
        order_service_part.order_service_id,
        order_service_part.quantity
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_order_service_part(
    pool: &DbPool,
    quantity: Option<i32>,
    order_service_part_id: i32,
) -> Result<OrderServicePart, DbError> {
    sqlx::query_as!(
        OrderServicePart,
        r#"
        UPDATE moto_auto.order_service_part
        SET
            quantity = COALESCE($1, quantity)
        WHERE order_service_part_id = $2
        RETURNING order_service_part_id, part_id, order_service_id, quantity
        "#,
        quantity,
        order_service_part_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_order_service_part(
    pool: &DbPool,
    order_service_part_id: i32,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.order_service_part
        WHERE order_service_part_id = $1
        "#,
        order_service_part_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_order_service_part(
    pool: &DbPool,
    part_id: Option<i32>,
    order_service_id: Option<i32>,
) -> Result<Vec<OrderServicePart>, DbError> {
    match (part_id, order_service_id) {
        (None, Some(id)) => {
            sqlx::query_as!(
                OrderServicePart,
                r#"
                SELECT * FROM moto_auto.order_service_part
                WHERE order_service_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(id), None) => {
            sqlx::query_as!(
                OrderServicePart,
                r#"
                SELECT * FROM moto_auto.order_service_part
                WHERE part_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(part_id), Some(order_service_id)) => {
            sqlx::query_as!(
                OrderServicePart,
                r#"
                SELECT * FROM moto_auto.order_service_part
                WHERE part_id = $1 AND order_service_id = $2
                "#,
                part_id,
                order_service_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                OrderServicePart,
                r#"
                SELECT * FROM moto_auto.order_service_part
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
