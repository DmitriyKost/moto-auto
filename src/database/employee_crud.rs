use bigdecimal::BigDecimal;
use crate::{database::{DbError, DbPool}, models::{BranchEmployee, Employee}};

pub async fn create_employee(pool: &DbPool, employee: Employee) -> Result<Employee, DbError> {
    sqlx::query_as!(
        Employee, 
        r#"
        INSERT INTO moto_auto.employee (name, age, position, contact_info, expirience_years, salary, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING employee_id, name, age, position, contact_info, expirience_years, salary, description
        "#,
        employee.name,
        employee.age,
        employee.position,
        employee.contact_info,
        employee.expirience_years,
        employee.salary,
        employee.description
        )
        .fetch_one(pool)
        .await.map_err(|e| DbError::Sqlx(e))
}

pub async fn create_branch_employee(pool: &DbPool, employee_id: i32, branch_id: i32) -> Result<BranchEmployee, DbError> {
    let new_city: String = sqlx::query_scalar!(
        r#"
        SELECT city 
        FROM moto_auto.branch 
        WHERE branch_id = $1
        "#,
        branch_id
    )
    .fetch_one(pool)
    .await.map_err(|e| DbError::Sqlx(e))?;

    let are_cities_same = sqlx::query_scalar!(
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
    .await.map_err(|e| DbError::Sqlx(e))?;

    if let Some(cities_same) = are_cities_same {
        if cities_same {
            return Err(DbError::SameCityError)
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
    .await.map_err(|e| DbError::Sqlx(e))
}

pub async fn update_employee(pool: &DbPool, name: Option<&str>, age: Option<i32>, position: Option<&str>, contact_info: Option<&str>, expirience_years: Option<i32>, salary: Option<BigDecimal>, description: Option<&str>, employee_id: i32) -> Result<Employee, DbError> {
    sqlx::query_as!(
        Employee,
        r#"
        UPDATE moto_auto.employee
        SET
            name = COALESCE($1, name),
            age = COALESCE($2, age),
            position = COALESCE($3, position),
            contact_info = COALESCE($4, contact_info),
            expirience_years = COALESCE($5, expirience_years),
            salary = COALESCE($6, salary),
            description = COALESCE($7, description)
        WHERE employee_id = $8
        RETURNING employee_id, name, age, position, contact_info, expirience_years, salary, description
        "#,
        name,
        age,
        position,
        contact_info,
        expirience_years,
        salary,
        description,
        employee_id
    )
    .fetch_one(pool)
    .await.map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_employee(pool: &DbPool, employee_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.employee
        WHERE employee_id = $1
        "#,
        employee_id
    )
    .execute(pool)
    .await.map_err(|e| DbError::Sqlx(e))
    .map(|_|{})

}

pub async fn delete_branch_employee(pool: &DbPool, employee_id: i32, branch_id: i32) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM moto_auto.branch_employee
        WHERE employee_id = $1 AND branch_id = $2
        "#,
        employee_id,
        branch_id
    )
    .execute(pool)
    .await.map_err(|e| DbError::Sqlx(e))
    .map(|_|{})

}

pub async fn get_employees(pool: &DbPool, branch_id: i32) -> Result<Vec<Employee>, DbError> {
    sqlx::query_as!(
        Employee,
        r#"
        SELECT e.* 
        FROM moto_auto.employee e
        INNER JOIN moto_auto.branch_employee be
        ON e.employee_id = be.employee_id
        WHERE be.branch_id = $1
        "#,
        branch_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}
