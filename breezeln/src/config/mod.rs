// Application Level Wide Configurations

use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub storage_dir: String,
    pub server_port: u16,
    pub mnemonic: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let api_key = env::var("SPARK_API_KEY").context("API KEY not set")?;

        let mnemonic = env::var("MNEMONIC").context("MNEMONIC must be provided")?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("SERVER_PORT must be a valid number")?;

        let storage_dir = env::var("STORAGE_DIR").context("ENCRYPTION_KEY not set")?;

        Ok(Config {
            api_key,
            mnemonic,
            server_port,
            storage_dir,
        })
    }
}
