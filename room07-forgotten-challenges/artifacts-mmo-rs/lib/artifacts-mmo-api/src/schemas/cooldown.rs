use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{CharacterSchema, ReasonSchema};

#[derive(Debug, Clone, Deserialize)]
pub struct CooldownSchema {
    pub total_seconds: u32,
    pub remaining_seconds: u32,
    pub started_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub reason: ReasonSchema,
}
