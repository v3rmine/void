use serde::Deserialize;

use super::{AnnouncementSchema, ResponseSchema};

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[derive(Debug, Clone, Deserialize)]
pub struct StatusSchema {
    pub status: String,
    pub version: String,
    pub characters_online: u32,
    pub announcements: Vec<AnnouncementSchema>,
    // REVIEW: not documented but might be dates
    pub last_wipe: String,
    pub next_wipe: String,
}
