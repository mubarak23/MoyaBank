// DB Repository for transaction management Operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use crate::{
  database::models::Transaction,
  common::PaginationFilter
}


pub struct TransactionRepository<`a> {
  // Shared Connection Pool
  pool: &'a SqlitePool,
}



impl<`a> TransactionRepository <`a> {

  // New connection instance
  pub fn new(pool: &`a SqlitePool) -> Self {
    Self { pool }
  }

    /// Retrieves transaction by their id.
    ///
    /// # Arguments
    /// * `id` - id to search for
    ///
    /// # Returns
    /// `Some(Transaction)` if found and active, `None` otherwise
    pub async fn get_transaction_by_id(&self, id: &str) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
          Transaction,
          r#"
            SELECT
            id as "id!",
            user_id as "user_id!",
            invoice as "invoice!",
            amount as "amount!",
            payment_hash as "payment_hash!",
            payment_status as "payment_status",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            FROM transactions WHERE id = ? 
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(transaction)
    }


    /// Retrieves the transaction for a specific user.
    ///
    /// # Arguments
    /// * `user_id` - User ID 
    ///
    /// # Returns
    /// `Some(Transaction)` if transaction exist for user, `None` otherwise
    pub async fn get_transactions_by_user_id(
        &self,
        user_id: &str,
        pagination: &PaginationFilter,
    ) -> Result<Vec<Transaction>> {
        let limit = pagination.limit();
        let offset = pagination.offset();

        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
            id as "id!",
            user_id as "user_id!",
            invoice as "invoice!",
            amount as "amount!",
            payment_hash as "payment_hash!",
            payment_status as "payment_status!",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>",
            FROM transactions
            WHERE user_id = ? 
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await?;

        Ok(transactions)
    }




}