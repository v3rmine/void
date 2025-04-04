use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: String,
    pub auction: String,
    pub user: String,
    pub value: u32,
    pub created: String,
    pub updated: String
}