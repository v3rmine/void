use serde::{Serialize, Deserialize};

use super::{IconFormat, AnimationFormat, Element};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NemesisEphemera {
    pub id: String,
    pub url_name: String,
    pub icon: String,
    pub icon_format: IconFormat,
    pub thumb: String,
    pub animation: String,
    pub animation_format: AnimationFormat,
    pub element: Element,
    pub item_name: String,
}