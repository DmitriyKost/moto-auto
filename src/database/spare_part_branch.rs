use bigdecimal::BigDecimal;

use crate::database::{DbError, DbPool};
use crate::models::SparePartBranch;

pub async fn create_spare_part_branch(pool: &DbPool, spare_part_branch: SparePartBranch) -> Result<SparePartBranch, DbError> {
    sqlx::query_as!(
        SparePartBranch,
        r#"
        INSERT INTO moto_auto.spare_part_branch (part_id, branch_id, stock_quantity, price)
        VALUES ($1, $2, $3, $4)
        RETURNING spare_part_branch_id, part_id, branch_id, stock_quantity, price
        "#,
        spare_part_branch.part_id,
        spare_part_branch.branch_id,
        spare_part_branch.stock_quantity,
        spare_part_branch.price
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_spare_part_branch(
    pool: &DbPool,
    stock_quantity: Option<i32>,
    price: Option<BigDecimal>,
    spare_part_branch_id: i32,
) -> Result<SparePartBranch, DbError> {
    sqlx::query_as!(
        SparePartBranch,
        r#"
        UPDATE moto_auto.spare_part_branch
        SET
            stock_quantity = COALESCE($1, stock_quantity),
            price = COALESCE($2, price)
        WHERE spare_part_branch_id = $3
        RETURNING spare_part_branch_id, part_id, branch_id, stock_quantity, price
        "#,
        stock_quantity,
        price,
        spare_part_branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_spare_part_branch(pool: &DbPool, part_id: i32, branch_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.spare_part_branch
        WHERE part_id = $1 AND branch_id = $2
        "#,
        part_id,
        branch_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_spare_part_branch(
    pool: &DbPool,
    branch_id: Option<i32>,
    part_id: Option<i32>,
) -> Result<Vec<SparePartBranch>, DbError> {
    match (branch_id, part_id) {
        (None, Some(id)) => {
            sqlx::query_as!(
                SparePartBranch,
                r#"
                SELECT * FROM moto_auto.spare_part_branch
                WHERE part_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(id), None) => {
            sqlx::query_as!(
                SparePartBranch,
                r#"
                SELECT * FROM moto_auto.spare_part_branch
                WHERE branch_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(part_id)) => {
            sqlx::query_as!(
                SparePartBranch,
                r#"
                SELECT * FROM moto_auto.spare_part_branch
                WHERE branch_id = $1 AND part_id = $2
                "#,
                branch_id,
                part_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                SparePartBranch,
                r#"
                SELECT * FROM moto_auto.spare_part_branch
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
