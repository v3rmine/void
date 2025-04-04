use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<'a, T> {
    pub status_code: u16,
    pub error_msg: Option<&'a str>,
    pub error_details: Option<&'a str>,
    #[serde(flatten)]
    pub value: GenericValue<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseTypes {
    #[serde(rename(serialize = "value"))]
    Boolean(bool),
    #[serde(rename(serialize = "value"))]
    Cookies(UserCookies),
}

pub type GenericValue<T> = Option<T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCookies {
    pub nc_session_id: String,
    pub nc_token: String,
    pub nc_username: String,
}
