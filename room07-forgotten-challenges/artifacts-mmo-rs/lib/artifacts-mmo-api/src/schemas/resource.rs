use serde::Deserialize;

use super::{DropRateSchema, SkillSchema};

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize)]
pub struct ResourceSchema {
    pub name: String,
    pub code: String,
    pub skill: SkillSchema,
    pub level: u32,
    pub drops: Vec<DropRateSchema>,
}
