// API Route handler for user related Endpoints
use sqlx::PgPool;
use crate::common::common::ApiResponse;
use crate::db::models::{User, UserWithAccount, CreateUser};
use crate::service::user_service::UserService;
use crate::common::common::service_error_to_http;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Json as ResponseJson,
};

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