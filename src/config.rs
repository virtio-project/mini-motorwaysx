use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, fs};

pub static CONFIG: Lazy<Config> = Lazy::new(Config::init);

/// # configuration
/// It loads configuration file path by reading env var `CONFIG`.
/// The default path is `${pwd}/config.toml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub actix: ActixConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActixConfig {
    pub bind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub screenshots: String,
}

impl Config {
    fn init() -> Self {
        let config_path = env::var("CONFIG").unwrap_or_else(|_| "config.toml".to_string());
        toml::from_str(
            fs::read_to_string(config_path)
                .expect("cannot read config file")
                .as_str(),
        )
        .unwrap()
    }
}

impl Default for ActixConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:8080".to_string(),
        }
    }
}
