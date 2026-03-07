use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::handlers;

pub struct AppRouter {
    pub router: Router,
}

impl Default for AppRouter {
    fn default() -> Self {
        let router = Router::new()
            // .merge(swagger_ui)
            .route("/", get(handlers::template::html_template))
            .route("/users", get(handlers::template::users_page))
            .route("/hello", get(handlers::template::hello_world))
            .route("/user-form", get(handlers::template::user_form))
            .route("/api/load", get(handlers::api::load_json_placeholder))
            .route("/api/posts/{id}", get(handlers::api::get_post_data))
            .route("/api/json", get(handlers::api::get_json))
            .route("/api/create-user", post(handlers::api::create_user))
            .fallback_service(ServeDir::new("./static"));

        Self { router }
    }
}
