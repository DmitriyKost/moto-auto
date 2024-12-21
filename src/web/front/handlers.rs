use crate::web::front::views::AdminIndex;
pub async fn admin_index() -> AdminIndex {
    AdminIndex {
        users: vec!["Jeka", "Vano", "Dimasik"],
    }
}
