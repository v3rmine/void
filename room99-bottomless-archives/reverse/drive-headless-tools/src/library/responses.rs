use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Response<'a> {
    pub code: u16,
    pub error: Option<&'a str>,
    pub p12_as_bytes: Option<&'a [u8]>,
    pub p12_as_base64: Option<&'a str>,
    pub json: Option<&'a str>,
}
