use serde::{Deserialize, Serialize};

// {"username":"foo","password":"bar","project_name":"some","sa_id":"x","response_type":"binary"}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Params<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub project_name: &'a str,
    pub sa_id: &'a str,
    pub response_type: &'a str,
    pub timeout: Option<u64>,
    pub headless: bool,
}
