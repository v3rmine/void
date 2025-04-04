#![allow(dead_code)]
pub const ERROR_400: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 400,
        err_short: Some("Bad Request"),
        err_long: Some("Bad parameters in the request!"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };

pub const ERROR_401: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 401,
        err_short: Some("Unauthorized"),
        err_long: Some("Please authenticate yourself correctly"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };

pub const ERROR_403: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 403,
        err_short: Some("Forbidden"),
        err_long: Some("Please make another request"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };

pub const ERROR_404: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 404,
        err_short: Some("Not Found"),
        err_long: Some("Route not found!"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };

pub const ERROR_405: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 405,
        err_short: Some("Method Not Allowed"),
        err_long: Some("This method is not allowed here!"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };

pub const ERROR_500: super::structs::Response<super::structs::ResponseTypes> =
    super::structs::Response {
        status_code: 500,
        err_short: Some("Internal Server Error"),
        err_long: Some("Internal server error please try again soon!"),
        value: None as super::structs::GenericResponse<super::structs::ResponseTypes>,
    };
