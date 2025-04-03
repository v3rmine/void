use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ItemEffectSchema {
    pub name: String,
    pub value: u32,
}
