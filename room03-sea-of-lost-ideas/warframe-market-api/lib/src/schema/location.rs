use serde::{Deserialize, Serialize};

use super::Faction;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub thumb: String,
    pub min_level: u32,
    pub max_level: u32,
    pub faction: Faction,
    pub node_name: String,
    pub system_name: String,
}