use serde::Deserialize;
use strum::Display;

/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_maps_maps__get>
#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MapContentTypeSchema {
    Monster,
    Resource,
    Workshop,
    Bank,
    GrandExchange,
    TasksMaster,
    Unknown(String),
}
