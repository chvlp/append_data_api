mod presenter;
mod model;
mod config;
mod handler;
mod utils;

use axum::{
    routing::post,
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::config::load_config;
use crate::handler::{create_data, create_field, get_data, get_field};

#[tokio::main]
async fn main() {
    let config = load_config();
    let addr = format!("{}:{}", config.app.url, config.app.port);


    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let middleware_stack = ServiceBuilder::new()
        .layer(cors);

    let app = Router::new()
        .route("/create-field", post(create_field))
        .route("/get-field", post(get_field))
        .route("/create-data", post(create_data))
        .route("/get-data", post(get_data))
        .layer(middleware_stack);


    println!("\n");
    println!("--------------------------------------");
    println!("running on http://{}", addr);
    println!("--------------------------------------\n");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
