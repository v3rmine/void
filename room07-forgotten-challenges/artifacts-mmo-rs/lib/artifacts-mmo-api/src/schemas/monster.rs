use serde::Deserialize;

use super::DropRateSchema;

#[derive(Debug, Clone, Deserialize)]
pub struct MonsterSchema {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub hp: u32,
    pub attack_fire: u32,
    pub attack_earth: u32,
    pub attack_water: u32,
    pub attack_air: u32,
    pub res_fire: u32,
    pub res_earth: u32,
    pub res_water: u32,
    pub res_air: u32,
    pub min_gold: u32,
    pub max_gold: u32,
    pub drops: Vec<DropRateSchema>,
}
