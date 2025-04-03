use serde::Deserialize;
use strum::Display;

#[derive(Debug, Clone, Deserialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ResultSchema {
    Win,
    Loss,
}
