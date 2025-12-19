// DB Repository for account management Operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use crate::{
  database::models::Account,
  common::PaginationFilter
}


pub struct AccountRepository<`a> {
  // Shared Connection Pool
  pool: &'a SqlitePool,
}



impl<`a> AccountRepository <`a> {

  // New connection instance
  pub fn new(pool: &`a SqlitePool) -> Self {
    Self { pool }
  }

    /// Retrieves an account by their id.
    ///
    /// # Arguments
    /// * `id` - id to search for
    ///
    /// # Returns
    /// `Some(Account)` if found and active, `None` otherwise
    pub async fn get_accoount_by_id(&self, account_id: &str) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
          Account,
          r#"
            SELECT
            id as "id!",
            user_id as "user_id!",
            balance as "balance!",
            is_active as "is_active!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: DateTime<Utc>"
            FROM accounts WHERE id = ? AND is_deleted = 0
            "#,
            account_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(account)
    }


    /// Retrieves an account by their user_id.
    ///
    /// # Arguments
    /// * `user_id` - user_id to search for
    ///
    /// # Returns
    /// `Some(Account)` if found and active, `None` otherwise
    pub async fn get_accoount_by_user_id(&self, user_id: &str) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
          Account,
          r#"
            SELECT
            id as "id!",
            user_id as "user_id!",
            balance as "balance!",
            is_active as "is_active!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: DateTime<Utc>"
            FROM accounts WHERE user_id = ? AND is_deleted = 0
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(account)
    }


}