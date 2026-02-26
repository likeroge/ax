use askama::Template;

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersListTemplate {
    pub users: Vec<String>,
}
