use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct InventorySlotSchema {
    pub slot: u32,
    pub code: String,
    pub quantity: u32,
}
