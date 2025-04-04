use crate::library::defaults;
use crate::library::methods;
use crate::library::{Response, ResponseTypes};
use actix_web::HttpResponse;
use actix_web_codegen::post;

#[post("/occ/restart")]
pub fn post_occ_restart() -> actix_web::Result<HttpResponse> {
    Ok(
        match || -> Result<(), Box<dyn std::error::Error>> {
            methods::unmount_all()?;
            let confs = methods::parse_config(defaults::CONFIG_PATH.to_owned())?;
            for conf in confs {
                methods::mount_config(conf.sa_id.as_str())?;
            }
            Ok(())
        }() {
            Ok(_) => response!(
                InternalServerError,
                Response::ok(200, ResponseTypes::PlainText("SA redemmarés".to_owned()))
            ),
            _ => response!(
                InternalServerError,
                Response::err(500, "Internal Server Error", "Error! Cannot delete the SA")
            ),
        },
    )
}
