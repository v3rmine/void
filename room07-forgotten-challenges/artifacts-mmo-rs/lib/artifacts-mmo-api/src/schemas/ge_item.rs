use serde::Deserialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_ge_item_ge__code__get>
#[derive(Debug, Clone, Deserialize)]
pub struct GEItemSchema {
    pub code: String,
    pub stock: u32,
    pub sell_price: u32,
    pub buy_price: u32,
}
