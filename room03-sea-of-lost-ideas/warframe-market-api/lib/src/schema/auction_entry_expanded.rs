use serde::{Serialize, Deserialize};

use super::{AuctionEntryItem, AuctionEntryIsMarkedFor, Platform, UserShort};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionEntry {
    pub id: String,
    pub minimal_reputation: u32,
    pub winner: Option<String>,
    pub private: bool,
    pub visible: bool,
    pub note_raw: String,
    pub note: String,
    pub owner: UserShort,
    pub starting_price: u32,
    pub buyout_price: Option<u32>,
    pub minimal_increment: u32,
    pub is_direct_sell: bool,
    pub top_bid: Option<u32>,
    pub created: String,
    pub updated: String,
    pub platform: Platform,
    pub closed: bool,
    pub is_marked_for: Option<AuctionEntryIsMarkedFor>,
    pub marked_operation_at: Option<String>,
    pub item: AuctionEntryItem
}