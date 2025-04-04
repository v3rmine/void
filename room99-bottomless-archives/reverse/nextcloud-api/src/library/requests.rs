use base64::encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct QueryCreateUser<'a> {
    pub command: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
// {"command":"get_token","username":"test","password":"test","value_type":"plaintext"}
pub struct QueryGetToken<'a> {
    pub command: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub value_type: Option<ResponseValueType>,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ResponseValueType {
    PlainText,
    Base64,
    UrlEncoded,
}

pub trait ConvertTo {
    fn convert_to(&self, _content: &str) -> String {
        unimplemented!()
    }
}

impl ConvertTo for Option<ResponseValueType> {
    fn convert_to(&self, content: &str) -> String {
        match self {
            Some(ResponseValueType::PlainText) => content.to_owned(),
            Some(ResponseValueType::Base64) => encode(content),
            Some(ResponseValueType::UrlEncoded) => super::urlencode(content),
            None => content.to_owned(),
        }
    }
}
