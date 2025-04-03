use serde::{Deserialize, Serialize};

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/action_equip_item_my__name__action_equip_post>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotTypeSchema {
    Weapon,
    Shield,
    Helmet,
    BodyArmor,
    LegArmor,
    Boots,
    Ring1,
    Ring2,
    Amulet,
    Artifact1,
    Artifact2,
    Artifact3,
    Consumable1,
    Consumable2,
    Unknown(String),
}
