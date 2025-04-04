use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelicDropData {
    pub r#ŧype: String,
    pub item: String,
    pub mission: String,
    pub location: String,
    pub rate: f32,
    pub rates: RelicDropDataRates,
    pub rarity: RelicDropDataRarity,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelicDropDataRarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    Legendary
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelicDropDataRates {
    pub intact: u32,
    pub exceptional: u32,
    pub flawless: u32,
    pub radiant: u32,
}