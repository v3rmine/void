use serde::{Deserialize, Serialize};

use super::Polarity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivenAuction {
    pub r#type: String,
    pub attributes: Vec<RivenAuctionAttribute>,
    pub name: String,
    pub mastery_level: u32,
    pub re_rolls: u32,
    pub weapon_url_name: String,
    pub polarity: Polarity,
    pub mod_rank: u32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivenAuctionAttribute {
    pub positive: bool,
    pub value: u32,
    pub url_name: String,
}