use serde::{Deserialize, Serialize};

pub type GenericResponse<T> = Option<T>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<'a> {
    pub status_code: u16,
    pub err_short: Option<&'a str>,
    pub err_long: Option<&'a str>,
    #[serde(flatten)]
    pub value: GenericResponse<ResponseTypes>,
}

impl<'a> Response<'a> {
    pub fn new(
        code: u16,
        eshort: Option<&'a str>,
        elong: Option<&'a str>,
        value: GenericResponse<ResponseTypes>,
    ) -> Response<'a> {
        Self {
            status_code: code,
            err_short: eshort,
            err_long: elong,
            value,
        }
    }
    #[allow(dead_code)]
    pub fn ok(code: u16, value: ResponseTypes) -> Response<'a> {
        match value {
            ResponseTypes::Null() => {
                Response::new(code, None, None, None as GenericResponse<ResponseTypes>)
            }
            _ => Response::new(code, None, None, Some(value)),
        }
    }
    #[allow(dead_code)]
    pub fn err(code: u16, eshort: &'a str, elong: &'a str) -> Response<'a> {
        Response::new(
            code,
            Some(eshort),
            Some(elong),
            None as GenericResponse<ResponseTypes>,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseTypes {
    #[serde(rename(serialize = "value"))]
    Boolean(bool),
    #[serde(rename(serialize = "value"))]
    PlainText(String),
    #[serde(rename(serialize = "value"))]
    Base64(String),
    Null(),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooke {
    pub nc_session_id: String,
    pub nc_token: String,
    pub nc_username: String,
}
