use serde::{Deserialize, Serialize};

pub type GenericResponse<T> = Option<T>;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Response<'a, T> {
    pub status_code: u16,
    pub err_short: Option<&'a str>,
    pub err_long: Option<&'a str>,
    #[serde(flatten)]
    pub value: GenericResponse<T>,
}

impl<'a, T> Response<'a, T> {
    pub fn new(
        code: u16,
        eshort: Option<&'a str>,
        elong: Option<&'a str>,
        value: GenericResponse<T>,
    ) -> Response<'a, T> {
        Self {
            status_code: code,
            err_short: eshort,
            err_long: elong,
            value,
        }
    }
    #[allow(dead_code)]
    pub fn ok(code: u16, value: T) -> Response<'a, T> {
        Response::new(code, None, None, Some(value))
    }
    #[allow(dead_code)]
    pub fn err(code: u16, eshort: &'a str, elong: &'a str) -> Response<'a, ResponseTypes> {
        Response::new(
            code,
            Some(eshort),
            Some(elong),
            Some(ResponseTypes::Null(None)),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextCloudCoke {
    pub nc_session_id: String,
    pub nc_token: String,
    pub nc_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccount<T> {
    pub json: Option<String>,
    pub p12: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseTypes {
    #[serde(rename(serialize = "value"))]
    Boolean(bool),
    #[serde(rename(serialize = "value"))]
    PlainText(String),
    #[serde(rename(serialize = "value"))]
    Base64(String),
    #[serde(rename(serialize = "value"))]
    SABinary(ServiceAccount<Vec<u8>>),
    #[serde(rename(serialize = "value"))]
    SABase64(ServiceAccount<String>),
    #[serde(rename(serialize = "value"))]
    Cookie(NextCloudCoke),
    #[serde(rename(serialize = "value"))]
    Null(Option<()>),
}
/*
pub trait ConvertTo {
    fn convert_to(&self, _content: &str) -> String {
        unimplemented!()
    }
}

impl ConvertTo for Option<ResponseTypes> {
    fn convert_to(&self, content: &str) -> String {
        match self {
            Some(ResponseTypes::PlainText) => content.to_owned(),
            Some(ResponseTypes::Base64) => encode(content),
            Some(ResponseTypes::UrlEncoded) => super::urlencode(content),
            None => content.to_owned(),
        }
    }
}
*/
