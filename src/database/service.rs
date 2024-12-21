use crate::database::{DbError, DbPool};
use crate::models::Service;

pub async fn create_service(pool: &DbPool, service: Service) -> Result<Service, DbError> {
    sqlx::query_as!(
        Service,
        r#"
        INSERT INTO moto_auto.service (service_name, description)
        VALUES ($1, $2)
        RETURNING service_id, service_name, description
        "#,
        service.service_name,
        service.description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_service(
    pool: &DbPool,
    service_name: Option<&str>,
    description: Option<&str>,
    service_id: i32,
) -> Result<Service, DbError> {
    sqlx::query_as!(
        Service,
        r#"
        UPDATE moto_auto.service
        SET
            service_name = COALESCE($1, service_name),
            description = COALESCE($2, description)
        WHERE service_id = $3
        RETURNING service_id, service_name, description
        "#,
        service_name,
        description,
        service_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_service(pool: &DbPool, service_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.service
        WHERE service_id = $1
        "#,
        service_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_service(pool: &DbPool) -> Result<Vec<Service>, DbError> {
    sqlx::query_as!(
        Service,
        r#"
        SELECT * FROM moto_auto.service
       "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}
