// User Service Logic 
//! Handles all account-related business operations

use sqlx::SqlitePool;
use Uuid::uuid;
use Validator::Validate;
use crate::errors::{ServiceError, ServiceResult};
use crate::database::models::{
  User, CreateUser, Role, CreateRole, Account, CreateAccount
};
use crate::repositories::user_repository::UserRepository;
use crate::repositories::account_repository::AccountRepository;
use crate::repositories::role_repository::RoleRepository;


// Service layer for User related Operation
pub struct UserService<`a> {
  pool: &`a SqlitePool,
}


impl <`a> UserService<`a> {
     /// Creates a new AccountService instance.
    ///
    /// # Arguments
    /// * `pool` - Reference to SQLite connection pool
    pub fn new(pool: &`a SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new user with full validation and setup.
    ///
    /// # Arguments
    /// * `create_user` - User creation data transfer object
    ///
    /// # Returns
    /// Combined `UserWithAccount` containing both the new account and admin user
    ///
    /// # Errors
    /// Returns `ServiceError` for:
    /// - Validation failures
    /// - Duplicate account names, usernames, or emails
    /// - Missing required roles
    /// - Business rule violations
    pub async fn create_user(&self, create_user: CreateUser) -> ServiceResult<UserWithAccount> {
      // input validation
      if let Err(validation_errors) = create_user.validate() {
          let error_messages: Vec<String> = validation_errors
              .field_errors()
              .into_iter()
              flat_map(|(field, errors)| {
                  errors.iter().map(move |error| {
                      format!(
                            "{}: {}",
                            field,
                            error.message.as_ref().unwrap_or(&"Invalid value".into())
                        )
                  })
              }).collect();
          return Err(ServiceError::validation(error_messages.join(", ")));
      }
      
      let user_repo = UserRepository::new(self.pool);
      let account_repo = AccountRepository::new(self.pool);
      let role_repo = RoleRepository::new(self.pool);

       // Check if username already exists
       if user_repo.username_exists(&create_user.username).await? {
          return Err(ServiceError::already_exists(
            "User with the username Exist",
            &create_user.username
          ))
       }

      // Check if username already exists
       if user_repo.email_exists(&create_user.email).await? {
          return Err(ServiceError::already_exists(
            "User with the email Exist",
            &create_user.email
          ))
       }

      // Check if role id does not exists
       if role_repo.role_repo(&create_user.role_id).await? {
          return Err(ServiceError::already_exists(
            "Role does not exist",
            &create_user.role_id
          ))
       }

        // Start a transaction for atomic account + user creation
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ServiceError::Database { source: e.into() })?;


       // create the user
        let password_hash = bcrypt::hash(&create_user.password, bcrypt::DEFAULT_COST)
            .map_err(|e| ServiceError::validation(format!("Password hashing failed: {e}")))?;
       
      let user_id = Uuid::now_v7().to_string();
       let user = sqlx::query_as!(
            crate::database::models::User,
            r#"
            INSERT INTO users (id, role_id, username, password_hash, email, is_active)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING
            id as "id!",
            role_id as "role_id!",
            username as "username!",
            password_hash as "password_hash!",
            email as "email!",
            is_active as "is_active!",
            created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: chrono::DateTime<chrono::Utc>"
            "#,
            user_id,
            create_user.role_id,
            create_user.username,
            password_hash,
            create_user.email,
            true
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            ServiceError::Database { source: e.into() }
        })?;
      
        
        let account_id = Uuid::now_v7().to_string();
        // Insert the account into the database
        let account = sqlx::query_as!(
            crate::database::models::Account,
            r#"
            INSERT INTO accounts (id, user_id, balance, is_active)
            VALUES (?, ?, ?, ?)
            RETURNING
            id as "id!",
            user_id as "user_id!",
            balance as "balance!",
            is_active as "is_active!",
            created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>",
            is_deleted as "is_deleted!",
            deleted_at as "deleted_at?: chrono::DateTime<chrono::Utc>"
            "#,
            account_id,
            &user.id,
            0
            true
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            ServiceError::Database { source: e.into() }
        })?;
      
        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| ServiceError::Database { source: e.into() })?;
        
        &user.password_hash = "";
        let user_with_account = UserWithAccount { account, user };

        Ok(user_with_account)

    }

}
