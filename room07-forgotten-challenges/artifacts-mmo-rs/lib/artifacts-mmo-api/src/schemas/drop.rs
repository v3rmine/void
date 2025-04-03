use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DropSchema {
    pub code: String,
    pub quantity: u32,
}
