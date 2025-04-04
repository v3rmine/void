use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubrowAuction {
    /// always "kubrow" here
    pub r#type: String,
    pub name: String
}