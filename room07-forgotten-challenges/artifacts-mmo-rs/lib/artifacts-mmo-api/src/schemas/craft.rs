use serde::Deserialize;

use super::{CraftSkillSchema, SimpleItemSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct CraftSchema {
    pub skill: CraftSkillSchema,
    pub level: u32,
    pub items: Vec<SimpleItemSchema>,
    pub quantity: u32,
}
