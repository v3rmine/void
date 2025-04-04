use serde::{Deserialize, Serialize};

use super::{OrderType, Platform, UserShort, ItemInOrder};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderFull {
    pub id: String,
    pub platinum: u32,
    pub quantity: u32,
    pub order_type: OrderType,
    pub platform: Platform,
    #[deprecated]
    pub region: String,
    pub creation_date: String,
    pub last_update: String,
    pub subtype: String,
    pub visible: bool,
    pub user: UserShort,
    pub item: ItemInOrder,
}