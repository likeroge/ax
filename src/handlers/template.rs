use askama::Template;
use axum::response::Html;

use crate::{
    errors::ApiError,
    responses::ApiResponse,
    template_structs::{common::IndexTemplate, users::UsersListTemplate},
};

pub async fn html_template() -> Result<ApiResponse, ApiError> {
    let template = IndexTemplate {
        name: "Hello world".to_string(),
    };
    Ok(ApiResponse::OkHtml(Html(template.render()?)))
}

pub async fn users_page() -> Result<ApiResponse, ApiError> {
    let templ = UsersListTemplate {
        users: vec![
            "User1".to_string(),
            "User2".to_string(),
            "User3".to_string(),
            "User4".to_string(),
            "User5".to_string(),
        ],
    };
    Ok(ApiResponse::OkHtml(Html(templ.render()?)))
}
