
//! Rust structs that represent database table mappings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(
        min = 1,
        max = 255,
        message = "User name must be between 1-255 characters"
    ))]
    pub username: String,
    #[validate(
        email(message = "Must be a valid email"),
        length(max = 255, message = "Email too long")
    )]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    #[validate(length(min = 1, message = "Role ID is required"))]
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithAccount {
    pub user: User,
    pub account: Account,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub balance: Number,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAccount {
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
    #[validate(length(min = 1, message = "Balance is required"))]
    pub balance: Number,
}



#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRole {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1-255 characters"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub invoice: String,
    pub amount: Number,
    pub payment_hash: String,
    pub payment_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTransaction {
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
    #[validate(length(min = 1, message = "Invoice is required"))]
    pub invoice: String,
    #[validate(length(min = 1, message = "Amount is required"))]
    pub amount: Number,
    #[validate(length(min = 1, message = "Payment Hash is required"))]
    pub payment_hash: String,
    #[validate(length(min = 1, message = "Payment Status is required"))]
    pub payment_status: String,
}