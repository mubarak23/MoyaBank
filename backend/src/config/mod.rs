// Application Level Wide Configurations

use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
  pub max_connections: u32,
  pub jwt_secret: String,
  pub jwt_expires_in_seconds: u64, 
  pub server_port: u16,
  pub encryption_key: String,
  pub acquire_timeout_seconds: u64,
  pub resent_api_key: String,
  pub from_email: String,
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Result<Self> {
    dotenvy::dotenv().ok();

    let max_connections = env::var("MAX_DB_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .context("Max db connection must be valid")?;
    let jwt_secret = env::var("JWT_SECRET").context("JWT_SECRET not set")?;

    let jwt_expires_in_seconds = env::var("JWT_EXPIRES_IN_SECONDS")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<u64>()
            .context("JWT_EXPIRES_IN_SECONDS must be a valid number")?;
    
    let database_url = env::var("DATABASE_URL").context("DATABASE URL must be provided")?;

    let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("SERVER_PORT must be a valid number")?;

   let encryption_key = env::var("ENCRYPTION_KEY").context("ENCRYPTION_KEY not set")?;

    let acquire_timeout_seconds = env::var("DB_ACQUIRE_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u64>()
            .context("DB_ACQUIRE_TIMEOUT_SECONDS must be a valid number")?;
    
    let resent_api_key = env::var("RESENT_API_KEY").context("RESENT_API_KEY not set")?;
    let from_email = env::var("FROM_EMAIL").context("FROM_EMAIL not set")?;

    Ok(Config {
      max_connections,
      jwt_secret,
      jwt_expires_in_seconds,
      database_url,
      server_port,
      encryption_key,
      acquire_timeout_seconds,
      resent_api_key,
      from_email
    })
  
  }
}