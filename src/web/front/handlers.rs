use axum::extract::Query;

use crate::models::User;
use crate::web::front::views::AdminIndex;

use super::views::UserEdit;

pub async fn admin_index() -> AdminIndex {
    AdminIndex {
        users: vec![User {
            user_id: Some(123),
            username: "Vano".to_string(),
            passwordhash: "Something".to_string(),
            role: "Master".to_string(),
            branch_id: 1358,
        }],
    }
}

pub async fn user_edit(Query(user): Query<User>) -> UserEdit {
    UserEdit { user }
}
