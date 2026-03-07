use std::collections::HashMap;

use axum::{extract::Path, response::Redirect, Form};
use serde_json::{json, Value};

use crate::{errors::ApiError, responses::ApiResponse};

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

pub async fn create_user(Form(user_name): Form<HashMap<String, String>>) -> Redirect {
    println!("{:?}", user_name);

    let redirect = Redirect::to("/");
    redirect
}

pub async fn get_post_data(Path(post_id): Path<u8>) -> Result<ApiResponse, ApiError> {
    println!("{}", post_id);
    Ok(ApiResponse::Ok)
}
