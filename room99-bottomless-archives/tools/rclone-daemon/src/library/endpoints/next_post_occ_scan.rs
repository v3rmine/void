use crate::library::{methods, requests::OnlyId};
use crate::library::{Response, ResponseTypes};
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::post;

#[post("/occ/scan")]
pub fn post_occ_scan(params: Json<OnlyId>) -> actix_web::Result<HttpResponse> {
    Ok(match methods::execute_occ_scan(params.id.as_str()) {
        Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Null())),
        Err(_) => response!(
            InternalServerError,
            Response::err(500, "Internal Server Error", "Error! Cannot scan that path")
        ),
    })
}
