use askama_axum::Template;

use crate::models::{Order, User};

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {}

#[derive(Template)]
#[template(path = "admin/base.html")]
pub struct AdminIndex {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "admin/user_edit.html")]
pub struct UserEdit {
    pub user: User,
}

#[derive(Template)]
#[template(path = "master/base.html")]
pub struct MasterIndex {
    pub orders: Vec<Order>,
}

#[derive(Template)]
#[template(path = "master/order_view.html")]
pub struct OrderEdit {
    pub order: Order,
}

#[derive(Template)]
#[template(path = "manager/base.html")]
pub struct ManagerIndex {
    pub orders: Vec<Order>,
}

#[derive(Template)]
#[template(path = "manager/order_edit.html")]
pub struct ManagerOrderView {
    pub order: Order,
}

#[derive(Template)]
#[template(path = "analyst/base.html")]
pub struct AnalystIndex {}
