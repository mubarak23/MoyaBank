// DB Repository for account management Operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::db::models::Account;
use sqlx::PgPool;


pub struct AccountRepository<'a> {
  // Shared Connection Pool
  pool: &'a PgPool,
}



impl<'a> AccountRepository <'a> {

  // New connection instance
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

    /// Retrieves an account by their id.
    ///
    /// # Arguments
    /// * 'id' - id to search for
    ///
    /// # Returns
    /// 'Some(Account)' if found and active, 'None' otherwise
pub async fn get_account_by_id(&self, account_id: &str) -> Result<Option<Account>> {
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
        FROM accounts
        WHERE id = $1
          AND is_deleted = false
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
    /// * 'user_id' - user_id to search for
    ///
    /// # Returns
    /// 'Some(Account)' if found and active, 'None' otherwise
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
            FROM accounts
            WHERE user_id = $1
              AND is_deleted = false
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(account)
    }


    /// Checks if a account already exists in the system.
    ///
    /// # Arguments
    /// * 'user_id' - user_id to check
    ///
    /// # Returns
    /// 'true' if a user with this username exists (and is not deleted)
    pub async fn account_exists(&self, user_id: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*)::BIGINT AS count
            FROM accounts
            WHERE user_id = $1
              AND is_deleted = false
            "#,
            user_id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(result.count.unwrap_or(0) > 0)
    }




}