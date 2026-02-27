mod errors;
mod handlers;
mod responses;
mod template_structs;

use std::net::SocketAddr;

use crate::handlers::api::ApiDoc;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let router = Router::new()
        // .merge(swagger_ui)
        .route("/", get(handlers::template::html_template))
        .route("/users", get(handlers::template::users_page))
        .route("/api/load", get(handlers::api::load_json_placeholder))
        .route("/api/posts/{id}", get(handlers::api::get_post_data))
        .route("/api/json", get(handlers::api::get_json))
        .fallback_service(ServeDir::new("./static"));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let tcp = TcpListener::bind(&addr).await.expect("Bind ip error");

    println!("Server started!");
    axum::serve(tcp, router).await.expect("Error start server");
}
