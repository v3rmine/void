use serde::Deserialize;
use strum::Display;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ReasonSchema {
    Movement,
    Fight,
    Crafting,
    Gathering,
    BuyGe,
    SellGe,
    DeleteItem,
    DepositBank,
    WithdrawBank,
    Equip,
    Unequip,
    Task,
    Recycling,
    Unknown(String),
}
