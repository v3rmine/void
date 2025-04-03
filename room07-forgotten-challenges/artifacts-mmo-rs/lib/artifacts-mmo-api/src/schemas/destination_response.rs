use serde::Deserialize;

use super::CharacterSchema;

#[derive(Debug, Clone, Deserialize)]
pub struct DestinationResponseSchema {
    pub name: String,
    pub x: u32,
    pub y: u32,
}
