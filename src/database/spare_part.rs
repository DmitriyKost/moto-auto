use crate::database::{DbError, DbPool};
use crate::models::SparePart;

pub async fn create_spare_part(pool: &DbPool, spare_part: SparePart) -> Result<SparePart, DbError> {
    sqlx::query_as!(
        SparePart,
        r#"
        INSERT INTO moto_auto.spare_part (part_name, description)
        VALUES ($1, $2)
        RETURNING part_id, part_name, description
        "#,
        spare_part.part_name,
        spare_part.description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_spare_part(
    pool: &DbPool,
    part_name: Option<&str>,
    description: Option<&str>,
    part_id: i32,
) -> Result<SparePart, DbError> {
    sqlx::query_as!(
        SparePart,
        r#"
        UPDATE moto_auto.spare_part
        SET
            part_name = COALESCE($1, part_name),
            description = COALESCE($2, description)
        WHERE part_id = $3
        RETURNING part_id, part_name, description
        "#,
        part_name,
        description,
        part_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_spare_part(pool: &DbPool, part_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.spare_part
        WHERE part_id = $1
        "#,
        part_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_spare_part(pool: &DbPool) -> Result<Vec<SparePart>, DbError> {
    sqlx::query_as!(
        SparePart,
        r#"
        SELECT * FROM moto_auto.spare_part
       "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}
