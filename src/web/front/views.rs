use askama_axum::Template;

#[derive(Template)]
#[template(path = "admin/base.html")]
pub struct AdminIndex {
    pub users: Vec<&'static str>,
}
