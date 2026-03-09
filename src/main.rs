mod app_router;
mod errors;
mod handlers;
mod responses;
mod template_structs;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::app_router::AppRouter;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // let router = Router::new().route("/", get(async || "Hello world"));
    let router = Router::new()
        // .merge(swagger_ui)
        .route("/", get(handlers::template::html_template))
        .route("/users", get(handlers::template::users_page))
        .route("/hello", get(handlers::template::hello_world))
        .route("/user-form", get(handlers::template::user_form))
        .route("/api/load", get(handlers::api::load_json_placeholder))
        .route("/api/posts/{id}", get(handlers::api::get_post_data))
        .route("/api/json", get(handlers::api::get_json))
        .route("/api/create-user", post(handlers::api::create_user));
    // .fallback_service(ServeDir::new("./static"));

    // let router = AppRouter::default().router;
    // let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp = TcpListener::bind(&addr).await.expect("Bind ip error");

    println!("Server started!");
    axum::serve(tcp, router).await.expect("Error start server");
}
