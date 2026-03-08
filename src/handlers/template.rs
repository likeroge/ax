use askama::Template;
use axum::response::Html;

use crate::{
    errors::ApiError,
    responses::ApiResponse,
    template_structs::{
        common::{HelloPageStruct, IndexTemplate},
        users::{UserFormTemplate, UsersListTemplate},
    },
};

pub async fn html_template() -> Result<ApiResponse, ApiError> {
    let template = IndexTemplate {
        message: "Hello world".to_string(),
    };

    Ok(ApiResponse::OkHtml(Html(template.render()?)))
}

pub async fn user_form() -> Result<ApiResponse, ApiError> {
    let template = UserFormTemplate {
        user_name: "".to_string(),
    };
    Ok(ApiResponse::OkHtml(Html(template.render()?)))
}

pub async fn hello_world() -> Result<ApiResponse, ApiError> {
    let template = HelloPageStruct {
        message: "Hello world".to_string(),
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
