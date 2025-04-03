use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockedHitsSchema {
    pub fire: u32,
    pub earth: u32,
    pub water: u32,
    pub air: u32,
    pub total: u32,
}
