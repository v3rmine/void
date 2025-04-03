use serde::Deserialize;

use super::{CharacterSchema, CooldownSchema, DestinationResponseSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterMovementDataSchema {
    pub cooldown: CooldownSchema,
    pub destination: DestinationResponseSchema,
    pub character: CharacterSchema,
}
