use serde::{Serialize, Deserialize};

use super::IconFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NemesisWeapon {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub icon_format: IconFormat,
    pub thumb: String,
    pub item_name: String
}