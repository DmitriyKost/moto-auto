use askama_axum::Template;

use crate::models::User;

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
