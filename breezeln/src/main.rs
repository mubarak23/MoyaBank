mod common;
mod config;
mod errors;
mod service;
mod state;
mod routes;

use crate::common::common::ApiResponse;
use axum::{response::Json, routing::get, Extension, Router};
use config::Config;
use state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();
    dotenvy::dotenv().ok();
    let config = Config::from_env().unwrap();

    let sdk = service::breeze::init_breeze(&config.storage_dir, &config.mnemonic, &config.api_key)
        .await
        .expect("failed to init breeze sdk");

    let state = AppState {
        breeze: Arc::new(sdk),
    };

    let app = Router::new().route("/", get(handle_root))
    .route("/list_payments", axum::routing::get(routes::invoice::list_payments))
    .route("/invoice", axum::routing::post(routes::invoice::create_invoice))
    .with_state(state);

    let bind_address = format!("0.0.0.0:{}", 3016);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    info!("Breeze server on port {}", 3015);
    axum::serve(listener, app).await.unwrap();

    println!("Breeze Service!");
}

async fn handle_root() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(
        serde_json::json!({
            "service": "Breeze API Backend",
            "version": "0.1.0"
        }),
        "Welcome to Breeze APIs",
    ))
}
