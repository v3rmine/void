use serde::Deserialize;

// {"username":"","password":""}
#[derive(Debug, Clone, Deserialize)]
pub struct NextCloudUser {
    pub username: String,
    pub password: String,
    pub response_type: Option<String>,
}

// {"username":"","password":"","project_name":"","sa_id":"","response_type":""}
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSA {
    pub username: String,
    pub password: String,
    pub project_name: String,
    pub sa_id: String,
    pub response_type: Option<String>,
}

// {"id":""}
#[derive(Debug, Clone, Deserialize)]
pub struct IdOnly {
    pub id: String,
}
