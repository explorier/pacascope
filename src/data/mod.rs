// Data ingestion and processing modules
pub mod log_parser;
pub mod portfolio;
pub mod cache;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyData {
    pub name: String,
    pub portfolio_value: f64,
    pub buying_power: f64,
    pub day_trades_used: u32,
    pub day_trades_remaining: u32,
    pub positions: Vec<Position>,
    pub recent_trades: Vec<Trade>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub market_value: f64,
    pub unrealized_pl: f64,
    pub unrealized_plpc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub side: String, // "buy" or "sell"
    pub quantity: f64,
    pub price: f64,
    pub strategy: String,
} 