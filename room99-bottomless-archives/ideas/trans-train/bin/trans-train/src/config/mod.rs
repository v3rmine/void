use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::lifecycle::Lifecycle;

mod lifecycle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub lifecycle: HashMap<String, Lifecycle>,
    pub user: HashMap<String, toml::Value>,
    pub plugins: HashMap<String, toml::Value>,
}
