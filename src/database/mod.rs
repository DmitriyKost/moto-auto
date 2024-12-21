pub mod user_crud;
pub mod branch_crud;
pub mod employee_crud;

use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

#[derive(Debug)]
pub enum DbError {
    Sqlx(sqlx::Error),
    NotPermitted,
    SameCityError
}

