use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserShort {
    pub id: String,
    pub ingame_name: String,
    pub status: Status,
    pub region: String,
    pub reputation: u32,
    pub avatar: Option<String>,
    pub last_seen: Option<String>
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ingame,
    Online,
    Offline
}