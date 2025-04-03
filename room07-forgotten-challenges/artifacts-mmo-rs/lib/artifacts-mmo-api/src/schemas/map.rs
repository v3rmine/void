use serde::Deserialize;

use super::MapContentSchema;

#[derive(Debug, Deserialize)]
pub struct MapSchema {
    pub name: String,
    pub skin: String,
    pub x: String,
    pub y: String,
    pub content: Option<MapContentSchema>,
}
