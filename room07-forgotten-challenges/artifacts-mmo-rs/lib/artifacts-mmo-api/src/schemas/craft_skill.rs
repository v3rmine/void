use serde::Deserialize;
use strum::Display;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_items_items__get>
#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum CraftSkillSchema {
    WeaponCrafting,
    GearCrafting,
    JewelryCrafting,
    Cooking,
    Woodcutting,
    Mining,
    Unknown(String),
}
