use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloPageStruct {
    pub message: String,
}
