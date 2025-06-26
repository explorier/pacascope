mod ui;
mod data;
mod config;

use anyhow::Result;
use apca::{ApiInfo, Client};
use apca::api::v2::account;
use config::AppConfig;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, time::Duration};
use ui::app::App;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Manually load .env file
    dotenv::dotenv().ok();
    
    println!("🦙 PacaScope - Paper Trading Monitor");
    println!("===================================");
    
    // Load configuration
    let app_config = AppConfig::load()?;
    println!("✓ Configuration loaded");
    
    // Try to load API configuration for all strategies
    println!("\n🔎 Checking API connections for all strategies...");
    let mut clients = HashMap::new();
    for strategy_config in &app_config.strategies {
        print!("  - Strategy '{}': ", strategy_config.name);
        match load_api_info_for_strategy(strategy_config) {
            Ok(api_info) => {
                let client = Client::new(api_info);
                if let Err(e) = test_api_connection(&client).await {
                    println!("Connection FAILED. Error: {:#}", e);
                } else {
                    println!("Connection OK");
                    clients.insert(strategy_config.name.clone(), client);
                }
            }
            Err(e) => {
                 println!("FAILED to load config ({})", e);
            }
        }
    }
    
    println!("\n✨ Loading initial account data...");
    let mut app = App::new(clients);
    load_initial_data(&mut app).await?;
    
    println!("\n📁 Checking for log files...");
    check_log_files(&app_config)?;
    
    println!("\n🚀 Starting TUI application...");
    
    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    
    Ok(())
}

fn load_api_info_for_strategy(config: &config::StrategyConfig) -> Result<ApiInfo> {
    let key_id = env::var(&config.key_id_var)
        .map_err(|e| anyhow::anyhow!("'{}': {}", &config.key_id_var, e))?;

    let secret_key = env::var(&config.secret_key_var)
        .map_err(|e| anyhow::anyhow!("'{}': {}", &config.secret_key_var, e))?;
    
    // All our strategies are paper trading for now.
    let url = "https://paper-api.alpaca.markets";
    
    Ok(ApiInfo::from_parts(url, key_id, secret_key)?)
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    while app.running {
        terminal.draw(|f| ui::render_main_layout(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    app.quit();
                }
            }
        }
    }
    Ok(())
}

async fn test_api_connection(client: &Client) -> Result<()> {
    // This is just a quick test, so we don't print the details anymore.
    client.issue::<account::Get>(&()).await?;
    Ok(())
}

async fn load_initial_data(app: &mut App) -> Result<()> {
    for (name, client) in &app.clients {
        print!("  - Fetching data for '{}'... ", name);
        match client.issue::<account::Get>(&()).await {
            Ok(account) => {
                let portfolio_value: f64 = account.equity.to_string().parse().unwrap_or(0.0);
                let buying_power: f64 = account.buying_power.to_string().parse().unwrap_or(0.0);

                let strategy_data = data::StrategyData {
                    name: name.clone(),
                    portfolio_value,
                    buying_power,
                    day_trades_used: account.daytrade_count as u32,
                    day_trades_remaining: 3 - account.daytrade_count as u32, // Simplified
                    positions: Vec::new(), // TODO
                    recent_trades: Vec::new(), // TODO
                    last_updated: chrono::Utc::now(),
                };
                app.strategies.insert(name.clone(), strategy_data);
                println!("OK");
            }
            Err(e) => {
                println!("FAILED. Error: {:#}", e);
            }
        }
    }
    Ok(())
}

fn check_log_files(config: &AppConfig) -> Result<()> {
    // This function will need to be refactored to use the new strategy config.
    // For now, let's just check if the parent log directory exists.
    let log_dir = std::path::Path::new("../pacabot/logs");
    
    if !log_dir.exists() {
        println!("⚠️  Log directory not found: {}", log_dir.display());
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
