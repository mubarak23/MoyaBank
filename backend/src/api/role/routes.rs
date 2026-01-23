//! Defines the HTTP routes for role management.

use super::handlers::create_role;

use axum::{Router, routing::post};

pub async fn role_router() -> Router {
    Router::new().route("/new_role", post(create_role))
}
