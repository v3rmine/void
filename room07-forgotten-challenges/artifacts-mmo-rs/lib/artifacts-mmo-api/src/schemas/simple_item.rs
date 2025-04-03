use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SimpleItemSchema {
    pub code: String,
    pub quantity: u32,
}
