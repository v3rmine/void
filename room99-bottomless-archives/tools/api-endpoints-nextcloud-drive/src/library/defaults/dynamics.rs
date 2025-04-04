#![allow(dead_code)]

pub type ErrorChainResult<T> = Result<T, Box<dyn std::error::Error>>;

type CustomErrorResult = super::structs::ResponseTypes;
pub fn custom_404(reason: &str) -> super::structs::Response<CustomErrorResult> {
    super::structs::Response {
        status_code: 400,
        err_short: Some("Bad Request"),
        err_long: Some(reason),
        value: None as super::structs::GenericResponse<CustomErrorResult>,
    }
}
