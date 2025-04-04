use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivenAttribute {
    pub id: String,
    pub url_name: String,
    pub group: RivenAttributeGroup,
    pub prefix: String,
    pub suffix: String,
    pub positive_is_negative: bool,
    pub exclusive_to: Option<RivenAttributeExclusiveTo>,
    pub effect: String,
    pub units: Option<RivenAttributeUnits>,
    pub negative_only: bool,
    pub search_only: bool
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RivenAttributeGroup {
    Default,
    Melee,
    Top
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RivenAttributeExclusiveTo {
    Kitgun,
    Pistol,
    Rifle,
    Shotgun,
    Melee,
    Zaws
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RivenAttributeUnits {
    Percent,
    Seconds
}