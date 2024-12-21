use crate::database::{DbError, DbPool};
use crate::models::Client;

pub async fn create_client(pool: &DbPool, client: Client) -> Result<Client, DbError> {
    sqlx::query_as!(
        Client,
        r#"
        INSERT INTO moto_auto.client (name, contact_info, status, bonus_points, total_spent)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING client_id, name, contact_info, status, bonus_points, total_spent
        "#,
        client.name,
        client.contact_info,
        client.status,
        client.bonus_points,
        client.total_spent
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_client(
    pool: &DbPool,
    name: Option<&str>,
    contact_info: Option<&str>,
    client_id: i32,
) -> Result<Client, DbError> {
    sqlx::query_as!(
        Client,
        r#"
        UPDATE moto_auto.client
        SET
            name = COALESCE($1, name),
            contact_info = COALESCE($2, contact_info)
        WHERE client_id = $3
        RETURNING client_id, name, contact_info, status, bonus_points, total_spent
        "#,
        name,
        contact_info,
        client_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_client(pool: &DbPool, client_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.client
        WHERE client_id = $1
        "#,
        client_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_clients_by_master_id(
    pool: &DbPool,
    master_id: i32,
) -> Result<Vec<Client>, DbError> {
    sqlx::query_as!(
        Client,
        r#"
        SELECT DISTINCT c.client_id, c.name, c.contact_info, c.status, c.bonus_points, c.total_spent
        FROM moto_auto.client c
        INNER JOIN moto_auto.orders o ON c.client_id = o.client_id
        WHERE o.master_id = $1
       "#,
        master_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn get_clients(
    pool: &DbPool,
    master_id: Option<i32>,
    status: Option<String>
) -> Result<Vec<Client>, DbError> {
    match (master_id, status) {
        (Some(id), None) => {
            sqlx::query_as!(
                Client,
                r#"
                SELECT DISTINCT c.client_id, c.name, c.contact_info, c.status, c.bonus_points, c.total_spent
                FROM moto_auto.client c
                INNER JOIN moto_auto.orders o ON c.client_id = o.client_id
                WHERE o.master_id = $1
               "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(id), Some(status)) => {
            sqlx::query_as!(
                Client,
                r#"
                SELECT DISTINCT c.client_id, c.name, c.contact_info, c.status, c.bonus_points, c.total_spent
                FROM moto_auto.client c
                INNER JOIN moto_auto.orders o ON c.client_id = o.client_id
                WHERE o.master_id = $1 AND c.status = $2
               "#,
                id,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, Some(status)) => {
            sqlx::query_as!(
                Client,
                r#"
                SELECT * FROM moto_auto.client
                WHERE status = $1
               "#,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                Client,
                r#"
                SELECT * FROM moto_auto.client
               "#
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
