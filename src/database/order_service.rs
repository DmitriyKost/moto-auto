use crate::database::{DbError, DbPool};
use crate::models::OrderService;

pub async fn create_order_service(pool: &DbPool, order_service: OrderService) -> Result<OrderService, DbError> {
    sqlx::query_as!(
        OrderService,
        r#"
        INSERT INTO moto_auto.order_service (order_id, service_id)
        VALUES ($1, $2)
        RETURNING order_service_id, order_id, service_id
        "#,
        order_service.order_id,
        order_service.service_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_order_service(pool: &DbPool, order_id: i32, service_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.order_service
        WHERE order_id = $1 AND service_id = $2
        "#,
        order_id,
        service_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_order_service(
    pool: &DbPool,
    order_id: Option<i32>,
    service_id: Option<i32>,
) -> Result<Vec<OrderService>, DbError> {
    match (order_id, service_id) {
        (None, Some(id)) => {
            sqlx::query_as!(
                OrderService,
                r#"
                SELECT * FROM moto_auto.order_service
                WHERE service_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(id), None) => {
            sqlx::query_as!(
                OrderService,
                r#"
                SELECT * FROM moto_auto.order_service
                WHERE order_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                OrderService,
                r#"
                SELECT * FROM moto_auto.order_service
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(order_id), Some(service_id)) => {
            sqlx::query_as!(
                OrderService,
                r#"
                SELECT * FROM moto_auto.order_service
                WHERE order_id = $1 AND service_id = $2
                "#,
                order_id,
                service_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
