use serde::{Deserialize, Serialize};

// {"sa_id":""} -> {"status_code":200,"error_msg":null,"error_details":null,"value":null}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConf {
    pub r#type: Option<String>,
    pub sa_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: Option<String>,
    pub service_account_file: String,
    pub trashed_only: Option<bool>,
    pub use_trash: Option<bool>,
    pub shared_with_me: Option<bool>,
    pub list_chunk: Option<u32>,
    pub chunk_size: Option<u32>,
    pub pacer_burst: Option<u32>,
    pub team_drive: Option<String>
}

// {"sa_id":""} -> {"status_code":200,"error_msg":null,"error_details":null,"value":null}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlyId {
    pub id: String,
}

// {"path":""} -> {"status_code":200,"error_msg":null,"error_details":null,"value":null}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathScan {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
