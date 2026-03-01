use askama::Template;

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersListTemplate {
    pub users: Vec<String>,
}

#[derive(Template)]
#[template(path = "user_form.html")]
pub struct UserFormTemplate {
    pub user_name: String,
}
