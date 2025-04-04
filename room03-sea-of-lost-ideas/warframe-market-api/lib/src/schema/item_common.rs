use serde::{Serialize, Deserialize};

use super::IconFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCommon {
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
}