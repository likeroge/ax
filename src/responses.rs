use axum::{
    Json, http,
    response::{Html, IntoResponse},
};
use serde_json::Value;

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Value),
    OkHtml(Html<String>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (http::StatusCode::OK).into_response(),
            Self::Created => (http::StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (http::StatusCode::OK, Json(data)).into_response(),
            Self::OkHtml(template) => (http::StatusCode::OK, Html(template)).into_response(),
        }
    }
}
