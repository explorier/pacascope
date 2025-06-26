// Alpaca API integration
pub mod alpaca;
pub mod client;

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub api_key: String,
    pub secret_key: String,
    pub paper: bool,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok(); // Load .env file if it exists
        
        let api_key = std::env::var("ALPACA_API_KEY")
            .or_else(|_| std::env::var("APCA_API_KEY_ID"))
            .map_err(|_| anyhow::anyhow!("ALPACA_API_KEY or APCA_API_KEY_ID not found in environment"))?;
            
        let secret_key = std::env::var("ALPACA_SECRET_KEY")
            .or_else(|_| std::env::var("APCA_API_SECRET_KEY"))
            .map_err(|_| anyhow::anyhow!("ALPACA_SECRET_KEY or APCA_API_SECRET_KEY not found in environment"))?;

        let paper = std::env::var("ALPACA_PAPER")
            .unwrap_or_else(|_| "true".to_string())
            .parse()
            .unwrap_or(true);

        let base_url = if paper {
            "https://paper-api.alpaca.markets".to_string()
        } else {
            "https://api.alpaca.markets".to_string()
        };

        Ok(Self {
            base_url,
            api_key,
            secret_key,
            paper,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub account_number: String,
    pub status: String,
    pub currency: String,
    pub buying_power: String,
    pub cash: String,
    pub portfolio_value: String,
    pub equity: String,
    pub last_equity: String,
    pub multiplier: String,
    pub daytrade_count: i32,
} 