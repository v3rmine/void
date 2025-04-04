use crate::library::{methods, requests::User};
use crate::library::{Response, ResponseTypes};
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::post;

#[post("/occ")]
pub fn post_occ(params: Json<User>) -> actix_web::Result<HttpResponse> {
    Ok(
        match methods::create_occ_user(params.username.as_str(), params.password.as_str()) {
            Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Null())),
            Err(_) => response!(
                InternalServerError,
                Response::err(500, "Internal Server Error", "Error! Cannot scan that path")
            ),
        },
    )
}
