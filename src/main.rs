mod ui;
mod data;
mod api;
mod config;

use anyhow::Result;
use config::AppConfig;
use api::ApiConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🦙 PacaScope - Paper Trading Monitor");
    println!("===================================");
    
    // Load configuration
    let app_config = AppConfig::load()?;
    println!("✓ Configuration loaded");
    
    // Try to load API configuration
    match ApiConfig::from_env() {
        Ok(api_config) => {
            println!("✓ Alpaca API configuration loaded");
            println!("  - Base URL: {}", api_config.base_url);
            println!("  - Paper Trading: {}", api_config.paper);
            
            // Test API connection
            if let Err(e) = test_api_connection(&api_config).await {
                println!("⚠️  API connection test failed: {}", e);
                println!("   Make sure to copy .env from pacabot directory");
            } else {
                println!("✓ API connection test successful");
            }
        }
        Err(e) => {
            println!("⚠️  Alpaca API configuration not found: {}", e);
            println!("   Create a .env file with your Alpaca API keys:");
            println!("   ALPACA_API_KEY=your_key_here");
            println!("   ALPACA_SECRET_KEY=your_secret_here");
            println!("   ALPACA_PAPER=true");
        }
    }
    
    println!("\n📁 Checking for log files...");
    check_log_files(&app_config)?;
    
    println!("\n🚀 Starting TUI application...");
    // TODO: Start TUI application
    
    Ok(())
}

async fn test_api_connection(config: &ApiConfig) -> Result<()> {
    let client = reqwest::Client::new();
    
    let response = client
        .get(&format!("{}/v2/account", config.base_url))
        .header("APCA-API-KEY-ID", &config.api_key)
        .header("APCA-API-SECRET-KEY", &config.secret_key)
        .send()
        .await?;
    
    if response.status().is_success() {
        let account: api::AccountInfo = response.json().await?;
        println!("  - Account ID: {}", account.id);
        println!("  - Buying Power: ${}", account.buying_power);
        println!("  - Day Trades Used: {}", account.daytrade_count);
    } else {
        return Err(anyhow::anyhow!("API request failed with status: {}", response.status()));
    }
    
    Ok(())
}

fn check_log_files(config: &AppConfig) -> Result<()> {
    let log_dir = std::path::Path::new(&config.log_paths.log_directory);
    
    if !log_dir.exists() {
        println!("⚠️  Log directory not found: {}", config.log_paths.log_directory);
        println!("   Make sure pacabot is running in the parent directory");
        return Ok(());
    }
    
    let entries = std::fs::read_dir(log_dir)?;
    let mut log_files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name() {
            if let Some(name_str) = name.to_str() {
                if name_str.ends_with(".log") && 
                   (name_str.contains("tournament") || name_str.contains("pdt_safe")) {
                    log_files.push(name_str.to_string());
                }
            }
        }
    }
    
    if log_files.is_empty() {
        println!("⚠️  No trading log files found");
    } else {
        println!("✓ Found {} log files:", log_files.len());
        for file in log_files {
            println!("  - {}", file);
        }
    }
    
    Ok(())
}
