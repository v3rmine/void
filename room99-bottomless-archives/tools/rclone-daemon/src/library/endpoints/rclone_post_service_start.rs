use crate::library::methods;
use crate::library::requests::OnlyId;
use crate::library::{Response, ResponseTypes};
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::post;

#[post("/occ/start")]
pub fn post_occ_start(params: Json<OnlyId>) -> actix_web::Result<HttpResponse> {
    Ok(match methods::mount_config(params.id.as_str()) {
        Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Null())),
        Err(_) => response!(
            InternalServerError,
            Response::err(500, "Internal Server Error", "Error while mounting")
        ),
    })
}
