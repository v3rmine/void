use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GoldSchema {
    pub quantity: u32,
}
