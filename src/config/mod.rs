// Configuration management
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub refresh_interval_ms: u64,
    pub log_paths: LogPaths,
    pub api: ApiSettings,
    pub ui: UiSettings,
    pub alerts: AlertSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogPaths {
    pub tournament_alpha: String,
    pub tournament_beta: String,
    pub pdt_safe: String,
    pub log_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
    pub poll_interval_ms: u64,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub default_tab: String,
    pub chart_points: usize,
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub pdt_warning_threshold: u32, // Warn when this many day trades used
    pub drawdown_alert_percent: f64, // Alert when portfolio drops by this %
    pub enable_sound: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            refresh_interval_ms: 1000, // 1 second
            log_paths: LogPaths {
                tournament_alpha: "../pacabot/logs/tournament_alpha_*.log".to_string(),
                tournament_beta: "../pacabot/logs/tournament_beta_*.log".to_string(),
                pdt_safe: "../pacabot/logs/pdt_safe_*.log".to_string(),
                log_directory: "../pacabot/logs".to_string(),
            },
            api: ApiSettings {
                poll_interval_ms: 2000, // 2 seconds
                timeout_ms: 10000, // 10 seconds
                retry_attempts: 3,
            },
            ui: UiSettings {
                default_tab: "Alpha".to_string(),
                chart_points: 100,
                theme: "default".to_string(),
            },
            alerts: AlertSettings {
                pdt_warning_threshold: 2, // Warn at 2/3 day trades
                drawdown_alert_percent: 3.0, // 3% drawdown alert
                enable_sound: false,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Try to load from config file, fall back to default
        if let Ok(config_str) = std::fs::read_to_string("pacascope_config.toml") {
            toml::from_str(&config_str).map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_str = toml::to_string_pretty(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;
        std::fs::write("pacascope_config.toml", config_str)
            .map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;
        Ok(())
    }
} 