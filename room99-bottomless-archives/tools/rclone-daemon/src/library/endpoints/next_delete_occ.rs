use crate::library::{methods, requests::OnlyId};
use crate::library::{Response, ResponseTypes};
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::delete;

#[delete("/occ")]
pub fn delete_occ(params: Json<OnlyId>) -> actix_web::Result<HttpResponse> {
    Ok(match methods::delete_occ_user(params.id.as_str()) {
        Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Null())),
        Err(_) => response!(
            InternalServerError,
            Response::err(500, "Internal Server Error", "Error! Cannot scan that path")
        ),
    })
}
