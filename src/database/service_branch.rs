use bigdecimal::BigDecimal;

use crate::database::{DbError, DbPool};
use crate::models::ServiceBranch;

use super::service;

pub async fn create_service_branch(
    pool: &DbPool,
    service_branch: ServiceBranch,
) -> Result<ServiceBranch, DbError> {
    sqlx::query_as!(
        ServiceBranch,
        r#"
        INSERT INTO moto_auto.service_branch (price, branch_id, service_id)
        VALUES ($1, $2, $3)
        RETURNING service_branch_id, price, branch_id, service_id
        "#,
        service_branch.price,
        service_branch.branch_id,
        service_branch.service_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_service_branch(
    pool: &DbPool,
    price: Option<BigDecimal>,
    service_branch_id: i32,
) -> Result<ServiceBranch, DbError> {
    sqlx::query_as!(
        ServiceBranch,
        r#"
        UPDATE moto_auto.service_branch
        SET
            price = COALESCE($1, price)
        WHERE service_branch_id = $2
        RETURNING service_branch_id, price, branch_id, service_id
        "#,
        price,
        service_branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_service_branch(
    pool: &DbPool,
    service_id: i32,
    branch_id: i32,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.service_branch
        WHERE service_id = $1 AND branch_id = $2
        "#,
        service_id,
        branch_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_service_branch(
    pool: &DbPool,
    branch_id: Option<i32>,
    service_id: Option<i32>,
) -> Result<Vec<ServiceBranch>, DbError> {
    match (branch_id, service_id) {
        (None, Some(id)) => {
            sqlx::query_as!(
                ServiceBranch,
                r#"
                SELECT * FROM moto_auto.service_branch
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
                ServiceBranch,
                r#"
                SELECT * FROM moto_auto.service_branch
                WHERE branch_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(service_id)) => {
            sqlx::query_as!(
                ServiceBranch,
                r#"
                SELECT * FROM moto_auto.service_branch
                WHERE branch_id = $1 AND service_id = $2
                "#,
                branch_id,
                service_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                ServiceBranch,
                r#"
                SELECT * FROM moto_auto.service_branch
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
