use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dropsource {
    missions: Vec<DropsourceRelic>,
    relics: Vec<DropsourceMission>,
    npc: Vec<DropsourceNpc>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DropsourceMission {
    pub mission_id: String,
    pub node_id: String,
    pub rarity: DropsourceMissionRarity,
    pub rate: u32,
    pub item_subtype: DropsourceMissionItemSubtype,
    pub rotation: DropsourceMissionRotation,
    pub stage: DropsourceMissionStage
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DropsourceMissionRarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    Legendary
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DropsourceMissionItemSubtype {
    Intact,
    Exceptional,
    Flawless,
    Radiant,
    Small,
    Medium,
    Large,
    Basic,
    Adorned,
    Magnificient
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DropsourceMissionRotation {
    A,
    B,
    C
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DropsourceMissionStage {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "final")]
    Final
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DropsourceRelic {
    pub id: String,
    pub rarity: DropsourceRelicRarity,
    pub rate: DropsourceRelicRate,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DropsourceRelicRarity {
    Common,
    Uncommon,
    Rare
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DropsourceRelicRate {
    pub intact: u32,
    pub exceptional: u32,
    pub flawless: u32,
    pub radiant: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DropsourceNpc {
    pub id: String,
    pub rarity: DropsourceNpcRarity,
    pub rate: f32,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DropsourceNpcRarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    Legendary
}