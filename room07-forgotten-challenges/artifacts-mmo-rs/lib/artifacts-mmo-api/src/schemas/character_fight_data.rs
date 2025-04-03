use serde::Deserialize;

use super::{CharacterSchema, CooldownSchema, FightSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterFightDataSchema {
    pub cooldown: CooldownSchema,
    pub fight: FightSchema,
    pub character: CharacterSchema,
}
