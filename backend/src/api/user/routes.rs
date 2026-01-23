//! Defines the HTTP routes for user profile and management.

use super::handlers::{create_user, user_login};

use axum::{Router, routing::post};

pub async fn user_router() -> Router {
    Router::new()
        .route("/new_account", post(create_user))
        .route("/login", post(user_login))
}
