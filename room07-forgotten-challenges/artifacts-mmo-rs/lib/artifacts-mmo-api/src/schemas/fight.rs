use serde::Deserialize;

use super::{BlockedHitsSchema, DropSchema, ResultSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct FightSchema {
    pub xp: u32,
    pub gold: u32,
    pub drops: Vec<DropSchema>,
    pub turns: u32,
    pub monster_blocked_hits: BlockedHitsSchema,
    pub player_blocked_hits: BlockedHitsSchema,
    pub logs: Vec<String>,
    pub result: ResultSchema,
}
