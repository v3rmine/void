use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MissionDropData {
    pub r#ŧype: String,
    pub item: String,
    pub mission: String,
    pub location: String,
    pub rate: f32,
    pub rarity: MissionDropDataRarity,
    pub rotation: MissionDropDataRotation,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionDropDataRarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    Legendary
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionDropDataRotation {
    A,
    B,
    C,
}