// Role Service Logic
//! Handles all role related activities

use crate::db::models::{CreateRole, NewRole, Role};
use crate::errors::{ServiceError, ServiceResult};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::repositories::role_repository::RoleRepository;
use sqlx::types::BigDecimal;

// Service layer for Role related Operation
pub struct RoleService<'a> {
    pool: &'a PgPool,
}

impl<'a> RoleService<'a> {
    /// Creates a new role service instance.
    ///
    /// # Arguments
    /// * 'pool' - Reference to SQLite connection pool
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_role(&self, create_role: CreateRole) -> ServiceResult<NewRole> {
        // validate role payload
        if let Err(validation_errors) = create_role.validate() {
            let error_messages: Vec<String> = validation_errors
                .field_errors()
                .into_iter()
                .flat_map(|(field, errors)| {
                    errors.iter().map(move |error| {
                        format!(
                            "{}: {}",
                            field,
                            error.message.as_ref().unwrap_or(&"Invalid value".into())
                        )
                    })
                })
                .collect();
            return Err(ServiceError::validation(error_messages.join(", ")));
        }

        let role_repo = RoleRepository::new(self.pool);
        // check the role with the name exist
        if role_repo.role_exists_name(&create_role.name).await? {
            return Err(ServiceError::already_exists(
                "Role the provided name already exist",
                &create_role.name,
            ));
        }

        // Start a transaction for atomic account + user creation
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ServiceError::Database { source: e.into() })?;

        let role_id = Uuid::now_v7().to_string();
        let role = sqlx::query_as!(
            Role,
            r#"
          INSERT INTO roles (
              id,
              name,
              is_active
          )
          VALUES (
              $1, $2, $3
          )
          RETURNING
              id as "id!",
              name as "name!",
              is_active as "is_active!",
              created_at as "created_at!: chrono::DateTime<chrono::Utc>",
              updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
              is_deleted as "is_deleted!",
              deleted_at as "deleted_at?: chrono::DateTime<chrono::Utc>"
          "#,
            role_id,
            create_role.name,
            true
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ServiceError::Database { source: e.into() })?;

        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| ServiceError::Database { source: e.into() })?;

        let new_role = NewRole { role: role };
        Ok(new_role)
    }
}
