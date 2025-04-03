use serde::Deserialize;

use super::{CharacterSchema, CooldownSchema, ItemSchema, SlotTypeSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct EquipRequestSchema {
    pub cooldown: CooldownSchema,
    pub slot: SlotTypeSchema,
    pub item: ItemSchema,
    pub character: CharacterSchema,
}
