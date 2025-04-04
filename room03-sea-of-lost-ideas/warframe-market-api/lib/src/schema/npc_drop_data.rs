use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NpcDropData {
    pub r#ŧype: String,
    pub item: String,
    pub npc: String,
    pub rate: f32,
    pub rarity: NpcDropDataRarity
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NpcDropDataRarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    Legendary
}