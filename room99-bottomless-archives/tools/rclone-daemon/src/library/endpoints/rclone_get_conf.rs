use crate::library::defaults;
use crate::library::methods;
use crate::library::{Response, ResponseTypes};
use actix_web::HttpResponse;
use actix_web_codegen::get;

#[get("/conf")]
pub fn get_conf() -> actix_web::Result<HttpResponse> {
    Ok(match methods::dump_config(defaults::CONFIG_PATH) {
        Ok(x) => response!(Ok, Response::ok(200, ResponseTypes::PlainText(x))),
        Err(e) => {
            warn!("{}", e);
            response!(
                InternalServerError,
                Response::err(
                    500,
                    "Internal Server Error",
                    "Error! Cannot backup the config"
                )
            )
        }
    })
}
