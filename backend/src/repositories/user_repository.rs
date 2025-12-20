// DB Repository for user management Operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use crate::{
  database::models::User,
  common::PaginationFilter
}


pub struct UserRepository<`a> {
  // Shared Connection Pool
  pool: &'a SqlitePool,
}

impl<`a> UserRepository <`a> {

  // New connection instance
  pub fn new(pool: &`a SqlitePool) -> Self {
    Self { pool }
  }

    /// Checks if a username already exists in the system.
    ///
    /// # Arguments
    /// * `username` - Username to check
    ///
    /// # Returns
    /// `true` if a user with this username exists (and is not deleted)
    pub async fn username_exists(&self, username: &str) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = ? AND is_deleted = 0",
            username
        )
        .fetch_one(self.pool)
        .await?;

        Ok(count.count > 0)
    }

    /// Checks if an email already exists in the system.
    ///
    /// # Arguments
    /// * `email` - Email to check
    ///
    /// # Returns
    /// `true` if a user with this email exists (and is not deleted)
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE email = ? AND is_deleted = 0",
            email
        )
        .fetch_one(self.pool)
        .await?;

        Ok(count.count > 0)
    }


    /// Retrieves a user by their username.
    ///
    /// # Arguments
    /// * `username` - Username to search for
    ///
    /// # Returns
    /// `Some(User)` if found and active, `None` otherwise
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
          User,
          r#"
            SELECT
            id as "id!",
            role_id as "role_id!",
            username as "username!",
            password_hash as "password_hash!",
            email as "email!",
            is_active as "is_active!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: DateTime<Utc>"
            FROM users WHERE username = ? AND is_deleted = 0
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Retrieves a user by their user id.
    ///
    /// # Arguments
    /// * `user id` - id  to search for
    ///
    /// # Returns
    /// `Some(User)` if found and active, `None` otherwise
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
          User,
          r#"
            SELECT
            id as "id!",
            role_id as "role_id!",
            username as "username!",
            password_hash as "password_hash!",
            email as "email!",
            is_active as "is_active!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: DateTime<Utc>"
            FROM users WHERE id = ? AND is_deleted = 0
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Checks if an email already exists in the system.
    ///
    /// # Arguments
    /// * `email` - Email to check
    ///
    /// # Returns
    /// `true` if a user with this email exists (and is not deleted)
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let count = sqlx::query!(
          "SELECT COUNT(*) as count FROM users WHERE email = ? AND is_deleted = 0",
          email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count > 0)
    }



}