use log::error;

use crate::models::BranchEmployee;

use crate::database::{DbError, DbPool};

pub async fn delete_branch_employee(
    pool: &DbPool,
    employee_id: i32,
    branch_id: i32,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.branch_employee
        WHERE employee_id = $1 AND branch_id = $2
        "#,
        employee_id,
        branch_id
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn create_branch_employee(
    pool: &DbPool,
    employee_id: i32,
    branch_id: i32,
) -> Result<BranchEmployee, DbError> {
    let new_city: String = sqlx::query_scalar!(
        r#"
        SELECT city 
        FROM moto_auto.branch 
        WHERE branch_id = $1
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))?;

    let are_cities_diff = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM moto_auto.branch_employee be
            JOIN moto_auto.branch b ON be.branch_id = b.branch_id
            WHERE be.employee_id = $1
              AND b.city <> $2
        )
        "#,
        employee_id,
        new_city
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))?;

    if let Some(different) = are_cities_diff {
        if different {
            error!("Tried to employ with different cities");
            return Err(DbError::BadInput);
        }
    }

    sqlx::query_as!(
        BranchEmployee,
        r#"
        INSERT INTO moto_auto.branch_employee (employee_id, branch_id)
        VALUES ($1, $2)
        RETURNING branch_employee_id, employee_id, branch_id
        "#,
        employee_id,
        branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn get_branch_employee(
    pool: &DbPool,
    branch_id: Option<i32>,
    employee_id: Option<i32>,
) -> Result<Vec<BranchEmployee>, DbError> {
    match (branch_id, employee_id) {
        (None, Some(id)) => {
            sqlx::query_as!(
                BranchEmployee,
                r#"
                SELECT * FROM moto_auto.branch_employee
                WHERE employee_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(id), None) => {
            sqlx::query_as!(
                BranchEmployee,
                r#"
                SELECT * FROM moto_auto.branch_employee
                WHERE branch_id = $1
                "#,
                id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (Some(branch_id), Some(employee_id)) => {
            sqlx::query_as!(
                BranchEmployee,
                r#"
                SELECT * FROM moto_auto.branch_employee
                WHERE branch_id = $1 AND employee_id = $2
                "#,
                branch_id,
                employee_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        (None, None) => {
            sqlx::query_as!(
                BranchEmployee,
                r#"
                SELECT * FROM moto_auto.branch_employee
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
