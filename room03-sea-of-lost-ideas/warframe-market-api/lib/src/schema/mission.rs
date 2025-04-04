use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mission {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub thumb: String,
    pub name: String
}