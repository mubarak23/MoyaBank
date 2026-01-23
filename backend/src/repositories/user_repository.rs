// DB Repository for user management operations

use anyhow::Result;
use sqlx::PgPool;

use crate::db::models::User;

pub struct UserRepository<'a> {
    /// Shared Postgres connection pool
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    /// Create a new repository instance
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Checks if a username already exists in the system.
    ///
    /// Returns `true` if a user with this username exists and is not deleted.
    pub async fn username_exists(&self, username: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)::BIGINT AS count
            FROM users
            WHERE username = $1
              AND is_deleted = false
            "#,
            username
        )
        .fetch_one(self.pool)
        .await?;

        Ok(result.count.unwrap_or(0) > 0)
    }

    /// Checks if an email already exists in the system.
    ///
    /// Returns `true` if a user with this email exists and is not deleted.
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)::BIGINT AS count
            FROM users
            WHERE email = $1
              AND is_deleted = false
            "#,
            email
        )
        .fetch_one(self.pool)
        .await?;

        Ok(result.count.unwrap_or(0) > 0)
    }

    /// Retrieves a user by their username.
    ///
    /// Returns `Some(User)` if found and active, `None` otherwise.
    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                role_id,
                username,
                password_hash,
                email,
                is_active,
                created_at,
                updated_at,
                is_deleted,
                deleted_at
            FROM users
            WHERE username = $1
              AND is_deleted = false
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Retrieves a user by their username.
    ///
    /// Returns `Some(User)` if found and active, `None` otherwise.
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                role_id,
                username,
                password_hash,
                email,
                is_active,
                created_at,
                updated_at,
                is_deleted,
                deleted_at
            FROM users
            WHERE email = $1
              AND is_deleted = false
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Retrieves a user by their user ID.
    ///
    /// Returns `Some(User)` if found and active, `None` otherwise.
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                role_id,
                username,
                password_hash,
                email,
                is_active,
                created_at,
                updated_at,
                is_deleted,
                deleted_at
            FROM users
            WHERE id = $1
              AND is_deleted = false
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }
}
