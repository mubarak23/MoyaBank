//! Main entry point for Moya Bank

mod common;
mod errors;

use axum::{Extension, Router, response::Json, routing::get};
use crate::common::common::ApiResponse;
use serde::Serialize;
use serde::Deserialize;
use tracing::info;
use chrono::{DateTime, Utc};
use tracing_subscriber::fmt::init;

#[tokio::main]

async fn main() {
    init();

    let app = Router::new()
    .route("/", get(handle_root));

    let bind_address = format!("0.0.0.0:{}", 3004);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    info!("Started Moyabank  server on port {}", 3004);
    axum::serve(listener, app).await.unwrap();

   //  println!("Hello, world!");
}


async fn handle_root() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(
        serde_json::json!({
            "service": "MoyaBank Backend",
            "version": "0.1.0"
        }),
        "Welcome to Moya bank APIs"
    ))
}