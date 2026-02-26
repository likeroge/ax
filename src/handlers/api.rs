use axum::extract::Path;
use serde_json::{Value, json};
use utoipa::OpenApi;

use crate::{errors::ApiError, responses::ApiResponse};

pub async fn hello_word() -> &'static str {
    "Hello World"
}

pub async fn get_json() -> ApiResponse {
    let json_data = json!({"msg":"this is json"});
    ApiResponse::JsonData(json_data)
}

pub async fn load_json_placeholder() -> Result<ApiResponse, ApiError> {
    match reqwest::get("https://jsonplaceholderS.typicode.com/users/1").await {
        Ok(resp) => {
            println!("{:?}", resp);
            match resp.json::<Value>().await {
                Ok(data) => Ok(ApiResponse::JsonData(data)),
                Err(e) => Err(ApiError::InternalServerError),
            }
        }
        Err(e) => Err(ApiError::SpecialError(json!(e.to_string()))),
    }
}

pub async fn get_post_data(Path(post_id): Path<u8>) -> Result<ApiResponse, ApiError> {
    println!("{}", post_id);
    Ok(ApiResponse::Ok)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // get_users,
        // create_user,
        // get_user_by_id,
    ),
    components(
        // schemas(User, CreateUserRequest)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
