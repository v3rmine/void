use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Npc {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub thumb: String,
    pub name: String
}