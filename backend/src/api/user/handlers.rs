// API Route handler for user related Endpoints
use crate::common::common::ApiResponse;
use crate::common::common::service_error_to_http;
use crate::db::models::{CreateUser, LoginResponse, User, UserLogin, UserWithAccount};
use crate::service::user_service::UserService;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use sqlx::PgPool;

#[axum::debug_handler]
pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<ResponseJson<ApiResponse<UserWithAccount>>, (StatusCode, String)> {
    tracing::info!("Creating new User");

    let service = UserService::new(&pool);

    match service.create_user(payload).await {
        Ok(account) => {
            tracing::debug!("User created successfully: {:?}", account);
            Ok(ResponseJson(ApiResponse::success(
                account,
                "User created successfully",
            )))
        }
        Err(error) => Err(service_error_to_http(error)),
    }
}

#[axum::debug_handler]
pub async fn user_login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UserLogin>,
) -> Result<ResponseJson<ApiResponse<LoginResponse>>, (StatusCode, String)> {
    tracing::info!("User Attempt Login");

    let service = UserService::new(&pool);

    match service.login(payload).await {
        Ok(response) => {
            tracing::info!("Login successful");
            Ok(ResponseJson(ApiResponse::success(
                response,
                "Login successful",
            )))
        }
        Err(error) => Err(service_error_to_http(error)),
    }
}
