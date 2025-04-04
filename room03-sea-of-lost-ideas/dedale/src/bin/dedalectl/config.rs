use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct OptionalConfig {
    access_token: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Config {
    access_token: String,
}

impl Config {
    pub fn parse() -> Self {
        let config_path =
            PathBuf::from(env::var("HOME").expect("env var HOME is not set")).join("config.toml");
        
        let optional_config = if config_path.exists() {
            let file = std::fs::read_to_string(&config_path).unwrap();
            toml::from_str::<OptionalConfig>(&file).unwrap()
        } else {
            OptionalConfig::default()
        };
        Self {
            access_token: optional_config.access_token.unwrap_or_default(),
            ..Default::default()
        }
    }
}

