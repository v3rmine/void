use chrono::{DateTime, Utc};
use serde::Deserialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[derive(Debug, Clone, Deserialize)]
pub struct AnnouncementSchema {
    pub message: String,
    pub created_at: DateTime<Utc>,
}
