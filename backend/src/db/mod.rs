// Module for initializing the database connection pool
use anyhow::Result;
use crate::config::Config;
use std::time::Duration;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub mod models;

pub struct Database {
  pub pool: PgPool,
}

impl Database {
  pub async fn new(config: Config) -> Result<Self> {
      let database_url = &config.database_url;

      let pool: PgPool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await?;
      
     Ok(Database{ pool })   
  }

  pub fn pool (&self) -> &PgPool {
    &self.pool 
  }

}


impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
          pool: self.pool.clone()
        }
    }
}
