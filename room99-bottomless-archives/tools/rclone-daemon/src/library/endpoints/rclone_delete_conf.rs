use crate::library::methods;
use crate::library::requests::OnlyId;
use crate::library::{Response, ResponseTypes};
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::delete;

#[delete("/conf")]
pub fn delete_conf(params: Json<OnlyId>) -> actix_web::Result<HttpResponse> {
    Ok(
        match || -> Result<(), ()> {
            methods::delete_config(params.id.as_str()).map_err(|_| ())?;
            Ok(())
        }() {
            Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Null())),
            Err(_) => response!(
                InternalServerError,
                Response::err(500, "Internal Server Error", "Error! Cannot delete the SA")
            ),
        },
    )
}
