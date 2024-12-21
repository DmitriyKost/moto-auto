use crate::database::{DbError, DbPool};
use crate::models::Schedule;

pub async fn create_schedule(pool: &DbPool, schedule: Schedule) -> Result<Schedule, DbError> {
    sqlx::query_as!(
        Schedule,
        r#"
        INSERT INTO moto_auto.schedule (client_id, branch_id, order_id, scheduled_datetime, status)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING schedule_id, client_id, branch_id, order_id, scheduled_datetime, status
        "#,
        schedule.client_id,
        schedule.branch_id,
        schedule.order_id,
        schedule.scheduled_datetime,
        schedule.status
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_schedule(
    pool: &DbPool,
    scheduled_datetime: Option<chrono::DateTime<chrono::Utc>>,
    status: Option<&str>,
    schedule_id: i32,
) -> Result<Schedule, DbError> {
    sqlx::query_as!(
        Schedule,
        r#"
        UPDATE moto_auto.schedule
        SET
            scheduled_datetime = COALESCE($1, scheduled_datetime),
            status = COALESCE($2, status)
        WHERE schedule_id = $3
        RETURNING schedule_id, client_id, branch_id, order_id, scheduled_datetime, status
        "#,
        scheduled_datetime,
        status,
        schedule_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_schedule(pool: &DbPool, schedule_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.schedule
        WHERE schedule_id = $1
        "#,
        schedule_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_schedule(
    pool: &DbPool,
    branch_id: Option<i32>,
    client_id: Option<i32>,
    status: Option<&str>,
) -> Result<Vec<Schedule>, DbError> {
    match (branch_id, client_id, status) {
        (Some(id), None, None) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
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
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE client_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None, Some(status)) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE status = $1
                "#,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(client_id), None) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE branch_id = $1 AND client_id = $2
                "#,
                branch_id,
                client_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), None, Some(status)) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE branch_id = $1 AND status = $2
                "#,
                branch_id,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, Some(client_id), Some(status)) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE client_id = $1 AND status = $2
                "#,
                client_id,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(client_id), Some(status)) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                WHERE branch_id = $1 AND client_id = $2 AND status = $3
                "#,
                branch_id,
                client_id,
                status
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None, None) => {
            sqlx::query_as!(
                Schedule,
                r#"
                SELECT * FROM moto_auto.schedule
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
