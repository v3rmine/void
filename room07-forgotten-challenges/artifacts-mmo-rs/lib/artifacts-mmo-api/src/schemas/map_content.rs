use serde::Deserialize;

use super::MapContentTypeSchema;

#[derive(Debug, Deserialize)]
pub struct MapContentSchema {
    pub r#type: MapContentTypeSchema,
    pub code: String,
}
