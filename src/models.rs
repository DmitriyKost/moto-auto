use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Option<i32>,
    pub username: String,
    pub passwordhash: String,
    pub role: String,
    pub branch_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Branch {
    pub branch_id: Option<i32>,
    pub address: String,
    pub phone_number: String,
    pub postal_code: String,
    pub employee_count: i32,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub employee_id: Option<i32>,
    pub name: String,
    pub age: i32,
    pub position: String,
    pub contact_info: String,
    pub expirience_years: i32,
    pub salary: f64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BranchEmployee {
    pub branch_employee_id: Option<i32>,
    pub employee_id: i32,
    pub branch_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Client {
    pub client_id: Option<i32>,
    pub name: String,
    pub contact_info: String,
    pub status: String,
    pub bonus_points: i32,
    pub total_spent: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub order_id: Option<i32>,
    pub client_id: i32,
    pub branch_id: i32,
    pub order_date: chrono::DateTime<chrono::Utc>,
    pub completion_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Service {
    pub service_id: Option<i32>,
    pub service_name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ServiceBranch {
    pub service_branch_id: Option<i32>,
    pub price: f64,
    pub branch_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SparePart {
    pub part_id: Option<i32>,
    pub part_name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SparePartBranch {
    pub spare_part_branch_id: Option<i32>,
    pub part_id: i32,
    pub branch_id: i32,
    pub stock_quantity: i32,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderService {
    pub order_service_id: Option<i32>,
    pub order_id: i32,
    pub service_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderServicePart {
    pub order_service_part_id: Option<i32>,
    pub part_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Schedule {
    pub schedule_id: Option<i32>,
    pub client_id: i32,
    pub branch_id: i32,
    pub order_id: i32,
    pub preffered_master_id: i32,
    pub scheduled_datetime: chrono::DateTime<chrono::Utc>,
    pub status: String,
}
