pub mod branch;
pub mod branch_employee;
pub mod client;
pub mod employee;
pub mod order_service;
pub mod order_service_part;
pub mod orders;
pub mod schedule;
pub mod service;
pub mod service_branch;
pub mod spare_part;
pub mod spare_part_branch;
pub mod user;

use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

#[derive(Debug)]
pub enum DbError {
    Sqlx(sqlx::Error),
    NotPermitted,
    BadInput,
}
