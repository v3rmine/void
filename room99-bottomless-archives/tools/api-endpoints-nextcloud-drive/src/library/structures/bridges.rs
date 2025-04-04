use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalResponse<'a, T> {
    pub status_code: u16,
    pub error_msg: Option<&'a str>,
    pub error_details: Option<&'a str>,
    pub value: super::GenericResponse<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveResponse<'a> {
    pub code: u16,
    pub error: Option<&'a str>,
    pub p12_as_base64: Option<String>,
    pub p12_as_bytes: Option<Vec<u8>>,
    pub json: Option<String>,
}
