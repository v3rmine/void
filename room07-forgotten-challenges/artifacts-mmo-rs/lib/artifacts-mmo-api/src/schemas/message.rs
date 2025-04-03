use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MessageSchema {
    pub message: String,
}
