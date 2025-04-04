use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemShort {
    pub id: String,
    pub url_name: String,
    pub thumb: String,
    pub item_name: String
}