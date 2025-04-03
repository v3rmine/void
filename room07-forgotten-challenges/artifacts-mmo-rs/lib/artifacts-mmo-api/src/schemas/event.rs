use chrono::{DateTime, Utc};
use serde::Deserialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize)]
pub struct EventSchema {
    pub name: String,
    pub previous_skin: String,
    pub duration: u32,
    pub expiration: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
