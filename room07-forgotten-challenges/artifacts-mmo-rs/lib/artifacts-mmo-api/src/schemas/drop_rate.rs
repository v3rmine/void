use serde::Deserialize;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_resources_resources__code__get>
#[derive(Debug, Clone, Deserialize)]
pub struct DropRateSchema {
    pub code: String,
    pub rate: u32,
    pub min_quantity: u32,
    pub max_quantity: u32,
}
