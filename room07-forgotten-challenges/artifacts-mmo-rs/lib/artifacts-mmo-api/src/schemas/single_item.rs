use serde::Deserialize;

use super::{GEItemSchema, ItemSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct SingleItemSchema {
    pub item: ItemSchema,
    pub ge: GEItemSchema,
}
