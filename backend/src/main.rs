//! Main entry point for Moya Bank

mod api;
mod common;
mod config;
mod db;
mod errors;
mod repositories;
mod service;

use crate::common::common::ApiResponse;
use axum::{Extension, Router, response::Json, routing::get};
use chrono::{DateTime, Utc};
use config::Config;
use db::Database;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing_subscriber::fmt::init;

#[tokio::main]

async fn main() {
    init();

    let config = Config::from_env().unwrap();
    let db = Database::new(config).await.unwrap();
    let pool = db.pool().clone();
    let app = Router::new()
        .route("/", get(handle_root))
        .nest("/api/user", api::user::routes::user_router().await)
        .nest("/api/role", api::role::routes::role_router().await)
        .layer(Extension(pool));

    let bind_address = format!("0.0.0.0:{}", 3005);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    info!("Started Moyabank  server on port {}", 3005);
    axum::serve(listener, app).await.unwrap();

    //  println!("Hello, world!");
}

async fn handle_root() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(
        serde_json::json!({
            "service": "MoyaBank Backend",
            "version": "0.1.0"
        }),
        "Welcome to Moya bank APIs",
    ))
}
