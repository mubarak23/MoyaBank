// User Service Logic
//! Handles all account-related business operations

use crate::Config;
use crate::db::models::UserWithAccount;
use crate::db::models::{Account, CreateUser, LoginResponse, User, UserInfo, UserLogin};
use crate::errors::{ServiceError, ServiceResult};
use crate::repositories::account_repository::AccountRepository;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::user_repository::UserRepository;
use crate::utilities::jwt::JwtUtils;
use bcrypt::verify;
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use uuid::Uuid;
use validator::Validate;

// Service layer for User related Operation
pub struct UserService<'a> {
    pool: &'a PgPool,
    // jwt_utils: JwtUtils,
    // config: Config,
}

impl<'a> UserService<'a> {
    /// Creates a new AccountService instance.
    ///
    /// # Arguments
    /// * 'pool' - Reference to SQLite connection pool
    pub fn new(pool: &'a PgPool) -> Self {
        // let config = Config::from_env();
        //  let jwt_utils = JwtUtils::new();
        Self { pool }
    }

    /// Creates a new user with full validation and setup.
    ///
    /// # Arguments
    /// * 'create_user' - User creation data transfer object
    ///
    /// # Returns
    /// Combined 'UserWithAccount' containing both the new account and admin user
    ///
    /// # Errors
    /// Returns 'ServiceError' for:
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

        let user_repo = UserRepository::new(self.pool);
        let account_repo = AccountRepository::new(self.pool);
        let role_repo = RoleRepository::new(self.pool);

        // Check if username already exists
        if user_repo.username_exists(&create_user.username).await? {
            return Err(ServiceError::already_exists(
                "User with the username Exist",
                &create_user.username,
            ));
        }

        // Check if username already exists
        if user_repo.email_exists(&create_user.email).await? {
            return Err(ServiceError::already_exists(
                "User with the email Exist",
                &create_user.email,
            ));
        }

        // Check if role id does not exists
        if !role_repo.role_exists(&create_user.role_id).await? {
            return Err(ServiceError::already_exists(
                "Role does not exist",
                &create_user.role_id,
            ));
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
            User,
            r#"
          INSERT INTO users (
              id,
              role_id,
              username,
              password_hash,
              email,
              is_active
          )
          VALUES (
              $1, $2, $3, $4, $5, $6
          )
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
        .map_err(|e| ServiceError::Database { source: e.into() })?;

        let account_id = Uuid::now_v7().to_string();
        // Insert the account into the database
        let account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (id, user_id, balance, is_active)
            VALUES ($1, $2, $3, $4)
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
            BigDecimal::from(0),
            true
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ServiceError::Database { source: e.into() })?;

        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| ServiceError::Database { source: e.into() })?;

        //  &user.password_hash = "";
        let user_with_account = UserWithAccount { account, user };

        Ok(user_with_account)
    }

    /// Authenticate user and generate JWT tokens with node credentials if available
    pub async fn login(&self, login_request: UserLogin) -> ServiceResult<LoginResponse> {
        // Validate input
        if let Err(validation_errors) = login_request.validate() {
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

        // Authenticate user using UserService

        let user = self
            .authenticate_user(&login_request.email, &login_request.password)
            .await?;

        // Get account information
        let account_repo = AccountRepository::new(self.pool);
        let account = account_repo
            .get_accoount_by_user_id(&user.id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Account", &user.id))?;

        // Check if account is active
        if !account.is_active {
            return Err(ServiceError::validation("Account is inactive".to_string()));
        }

        // Store user ID before potential moves
        let user_id = user.id.clone();
        let account_id = account.id.clone();
        let user_role_id = user.role_id.clone();

        // Get Role Information
        let role_repo = RoleRepository::new(self.pool);
        let role = role_repo
            .get_role_id(&user_role_id)
            .await?
            .ok_or_else(|| ServiceError::not_found("Role", &user_role_id))?;

        // Generate tokens with node credentials if available
        let jwt_utils = JwtUtils::new();
        let access_token =
            jwt_utils?.generate_token(user_id.clone(), account_id.clone(), role.name.clone())?;

        let refresh_token =
            JwtUtils::new()?.generate_refresh_token(user_id.clone(), user_role_id.clone())?;

        // Get expires_in from config
        let config = Config::from_env().unwrap();
        let expires_in = config.jwt_expires_in_seconds;

        let user_info = UserInfo {
            id: user_id,
            username: user.username,
            email: user.email,
            account_id,
            role: role.name.clone(),
        };

        Ok(LoginResponse {
            access_token: access_token.to_string(),
            refresh_token,
            user: user_info,
            expires_in,
        })
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> ServiceResult<User> {
        let user_repo = UserRepository::new(self.pool);
        // Get user by username
        let user = user_repo
            .get_user_by_email(email)
            .await?
            .ok_or_else(|| ServiceError::validation("Invalid credentials".to_string()))?;

        // Check if user is active
        if !user.is_active {
            return Err(ServiceError::validation("Invalid credentials".to_string()));
        }

        // Verify password
        if !self.verify_password(password, &user.password_hash)? {
            return Err(ServiceError::validation(
                "Invalid username or password".to_string(),
            ));
        }

        Ok(user)
    }

    fn verify_password(&self, password: &str, hash: &str) -> ServiceResult<bool> {
        verify(password, hash)
            .map_err(|e| ServiceError::validation(format!("Password verification failed: {e}")))
    }
}
