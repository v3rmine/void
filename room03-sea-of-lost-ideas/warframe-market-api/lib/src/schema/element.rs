use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    Impact,
    Heat,
    Cold,
    Electricity,
    Toxin,
    Magnetic,
    Radiation
}