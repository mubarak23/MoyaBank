// DB Repository for role management Operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use crate::{
  database::models::Role,
  common::PaginationFilter
}


pub struct RoleRepository<`a> {
  // Shared Connection Pool
  pool: &'a SqlitePool,
}



impl<`a> RoleRepository <`a> {

  // New connection instance
  pub fn new(pool: &`a SqlitePool) -> Self {
    Self { pool }
  }


    /// Checks if an role already exists in the system.
    ///
    /// # Arguments
    /// * `name` - Name to check
    ///
    /// # Returns
    /// `true` if a role with this name exists (and is not deleted)
    pub async fn role_exists(&self, name: &str) -> Result<bool> {
        let count = sqlx::query!(
          "SELECT COUNT(*) as count FROM roles WHERE name = ? AND is_deleted = 0",
          name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count > 0)
    }

    /// Checks if an role already exists in the system.
    ///
    /// # Arguments
    /// * `role_id` - User id to check
    ///
    /// # Returns
    /// `true` if a role with this name exists (and is not deleted)
    pub async fn role_exists(&self, role_id: &str) -> Result<bool> {
        let count = sqlx::query!(
          "SELECT COUNT(*) as count FROM roles WHERE name = ? AND is_deleted = 0",
          name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(!count.count > 0)
    }


    /// Retrieves a role by their id.
    ///
    /// # Arguments
    /// * `id` - id to search for
    ///
    /// # Returns
    /// `Some(Role)` if found and active, `None` otherwise
    pub async fn get_role_id(&self, id: &str) -> Result<Option<Role>> {
        let role = sqlx::query_as!(
          Role,
          r#"
            SELECT
            id as "id!",
            name as "name!",
            is_active as "is_active!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: DateTime<Utc>"
            FROM roles WHERE id = ? AND is_deleted = 0
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(role)
    }



}