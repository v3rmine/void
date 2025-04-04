use serde::{Deserialize, Serialize};

use super::IconFormat;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemInOrder {
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
    pub en: ItemInOrderI18n,
    pub ru: ItemInOrderI18n,
    pub ko: ItemInOrderI18n,
    pub fr: ItemInOrderI18n,
    pub de: ItemInOrderI18n,
    pub sv: ItemInOrderI18n,
    pub zh_hant: ItemInOrderI18n,
    pub zh_hans: ItemInOrderI18n,
    pub pt: ItemInOrderI18n,
    pub es: ItemInOrderI18n,
    pub pl: ItemInOrderI18n
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemInOrderI18n {
    item_name: String
}