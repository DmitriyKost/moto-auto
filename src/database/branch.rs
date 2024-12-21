use crate::{
    database::{DbError, DbPool},
    models::Branch,
};
use log::error;

pub async fn create_branch(pool: &DbPool, branch: Branch) -> Result<Branch, DbError> {
    let branch = sqlx::query_as!(
        Branch,
        r#"
        INSERT INTO moto_auto.branch (address, phone_number, postal_code, employee_count, city)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING branch_id, address, phone_number, postal_code, employee_count, city
        "#,
        branch.address,
        branch.phone_number,
        branch.postal_code,
        branch.employee_count,
        branch.city,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e));
    if let Err(_) = branch {
        error!("Error creating new branch");
        return branch;
    }
    let branch = branch.unwrap();
    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO moto_auto.users (username, passwordhash, role, branch_id)
        VALUES ($1, $2, $3, $4)
        "#,
        "admin",
        "admin", // TODO: default passwordhash!
        "admin",
        branch.branch_id
    )
    .execute(pool)
    .await
    {
        error!("Error creating default admin");
        return Err(DbError::Sqlx(e));
    }
    return Ok(branch);
}

pub async fn update_branch(
    pool: &DbPool,
    admin_branch_id: i32,
    address: Option<&str>,
    phone_number: Option<&str>,
    postal_code: Option<&str>,
    employee_count: Option<i32>,
) -> Result<Branch, DbError> {
    sqlx::query_as!(
        Branch,
        r#"
        UPDATE moto_auto.branch
        SET
            address = COALESCE($1, address),
            phone_number = COALESCE($2, phone_number),
            postal_code = COALESCE($3, postal_code),
            employee_count = COALESCE($4, employee_count)
        WHERE branch_id = $5
        RETURNING branch_id, address, phone_number, postal_code, employee_count, city
        "#,
        address,
        phone_number,
        postal_code,
        employee_count,
        admin_branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn get_branch(
    pool: &DbPool,
    city: Option<&str>,
) -> Result<Vec<Branch>, DbError> {
    match city {
        Some(city) => {
            sqlx::query_as!(
                Branch,
                r#"
                SELECT * FROM moto_auto.branch
                WHERE city = $1
                "#,
                city
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        },
        None => {
            sqlx::query_as!(
                Branch,
                r#"
                SELECT * FROM moto_auto.branch
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| DbError::Sqlx(e))
        }
    }
}
