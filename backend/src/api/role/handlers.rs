// API Route handler for user related Endpoints
use crate::common::common::ApiResponse;
use crate::common::common::service_error_to_http;
use crate::db::models::{CreateRole, NewRole};
use crate::service::role_service::RoleService;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use sqlx::PgPool;

#[axum::debug_handler]
pub async fn create_role(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateRole>,
) -> Result<ResponseJson<ApiResponse<NewRole>>, (StatusCode, String)> {
    tracing::info!("Creating new Role");

    let service = RoleService::new(&pool);

    match service.create_role(payload).await {
        Ok(role) => {
            tracing::debug!("Role created successfully: {:?}", role);
            Ok(ResponseJson(ApiResponse::success(
                role,
                "User created successfully",
            )))
        }
        Err(error) => Err(service_error_to_http(error)),
    }
}
