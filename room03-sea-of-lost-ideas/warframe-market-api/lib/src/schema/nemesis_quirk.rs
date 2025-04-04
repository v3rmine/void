use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NemeisQuirk {
    pub id: String,
    pub url_name: String,
    pub item_name: String,
    pub description: String,
    pub group: String,
}