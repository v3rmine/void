use serde::Deserialize;
use strum::Display;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_items_items__get>
#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ItemTypeSchema {
    Consumable,
    BodyArmor,
    Weapon,
    Resource,
    LegArmor,
    Helment,
    Boots,
    Shield,
    Amulet,
    Ring,
    Unknown(String),
}
