// Module for initializing the database connection pool
use anyhow::Result;
use crate::config::Config;
use std::time::Duration;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};


pub struct Database {
  pub pool: SqlitePool,
}

impl Database {
  pub async fn new(config: Config) -> Result<Self> {
      let database_url = &config.database_url;

      let pool = SqlitePoolOptions::new()
          .max_connections(config.max_connections)
          .acquire_timeout(Duration::from_sec(config.acquire_timeout))
          .connect(database_url)
          .await?;
      
     Ok(Database{ pool })   
  }

  pub fn pool (&self) -> &SqlitePool {
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
