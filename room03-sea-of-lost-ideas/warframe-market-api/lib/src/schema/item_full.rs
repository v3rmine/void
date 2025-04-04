use serde::{Deserialize, Serialize};

use super::{IconFormat, LangInItem};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemFull {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub icon_format: IconFormat,
    pub thumb: String,
    pub sub_icon: String,
    #[deprecated = "in the next api version will be renamed to max_rank"]
    #[serde(alias = "max_rank")] 
    pub mod_max_rank: u32,
    #[serde(alias = "mod_max_rank")] 
    pub max_rank: u32,
    pub subtypes: Vec<String>,
    pub tags: Vec<String>,
    pub ducats: u32,
    pub quantity_for_set: u32,
    pub set_root: bool,
    pub mastery_level: u32,
    pub rarity: ItemFullRarity,
    pub trading_tax: u32,
    pub en: LangInItem,
    pub ru: LangInItem,
    pub ko: LangInItem,
    pub fr: LangInItem,
    pub de: LangInItem,
    pub sv: LangInItem,
    pub zh_hant: LangInItem,
    pub zh_hans: LangInItem,
    pub pt: LangInItem,
    pub es: LangInItem,
    pub pl: LangInItem
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemFullRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
    Peculiar
}