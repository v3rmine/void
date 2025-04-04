use serde::{Serialize, Deserialize};

use super::Element;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LichAuction {
    /// always "lich" here
    pub r#type: String,
    pub weapon_url_name: String,
    pub element: Element,
    pub damage: u32,
    pub ephemera: bool,
    pub quirk: String,
    pub name: String
}