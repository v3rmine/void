use serde::{Serialize, Deserialize};

use super::IconFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivenItem {
    pub id: String,
    pub url_name: String,
    pub group: RivenItemGroup,
    pub r#type: RivenType,
    pub icon: String,
    pub icon_format: IconFormat,
    pub thumb: String,
    pub item_name: String
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RivenItemGroup {
    Primary,
    Secondary,
    Melee,
    Zaw,
    Sentinel,
    Archgun,
    Kitgun
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RivenType {
    Shotgun,
    Rifle,
    Pistol,
    Melee,
    Zaw,
    Kitgun
}
