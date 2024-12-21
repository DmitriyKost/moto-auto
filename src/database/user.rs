use log::error;

use crate::database::{DbError, DbPool};
use crate::models::User;

async fn admin_branch_check(
    pool: &DbPool,
    admin_branch_id: i32,
    username: &str,
) -> Result<(), DbError> {
    let branch_id = sqlx::query_scalar!(
        r#"
        SELECT branch_id FROM moto_auto.users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await;
    if branch_id.is_err() {
        error!("Error fetching branch_id with username: {}", username);
        return Err(DbError::Sqlx(branch_id.err().unwrap()));
    }
    let branch_id = branch_id.unwrap();
    if admin_branch_id != branch_id {
        return Err(DbError::NotPermitted);
    }
    Ok(())
}

pub async fn create_user(pool: &DbPool, admin_branch_id: i32, user: User) -> Result<User, DbError> {
    if let Err(err) = admin_branch_check(pool, admin_branch_id, user.username.as_str()).await {
        return Err(err);
    }
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO moto_auto.users (username, passwordhash, role, branch_id)
        VALUES ($1, $2, $3, $4)
        RETURNING user_id, username, passwordhash, role, branch_id
        "#,
        user.username,
        user.passwordhash,
        user.role,
        user.branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn update_user(
    pool: &DbPool,
    admin_branch_id: i32,
    username: &str,
    new_passwordhash: Option<&str>,
    role: Option<&str>,
    branch_id: Option<i32>,
) -> Result<User, DbError> {
    if let Err(err) = admin_branch_check(pool, admin_branch_id, username).await {
        return Err(err);
    }

    sqlx::query_as!(
        User,
        r#"
        UPDATE moto_auto.users
        SET
            role = COALESCE($2, role),
            branch_id = COALESCE($3, branch_id),
            passwordhash = COALESCE($4, passwordhash)
        WHERE username = $1
        RETURNING user_id, username, passwordhash, role, branch_id
        "#,
        username,
        role,
        branch_id,
        new_passwordhash
    )
    .fetch_one(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}

pub async fn delete_user(
    pool: &DbPool,
    username: &str,
    admin_branch_id: i32,
) -> Result<(), DbError> {
    if let Err(err) = admin_branch_check(pool, admin_branch_id, username).await {
        return Err(err);
    }

    sqlx::query!(
        r#"
        DELETE FROM moto_auto.users
        WHERE username = $1
        "#,
        username
    )
    .execute(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
    .map(|_| {})
}

pub async fn get_users(pool: &DbPool, admin_branch_id: i32) -> Result<Vec<User>, DbError> {
    sqlx::query_as!(
        User,
        r#"
        SELECT * FROM moto_auto.users
        WHERE branch_id = $1
       "#,
        admin_branch_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| DbError::Sqlx(e))
}
