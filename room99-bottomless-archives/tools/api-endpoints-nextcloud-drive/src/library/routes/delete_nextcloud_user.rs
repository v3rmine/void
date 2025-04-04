use super::{
    defaults, execute,
    structs::{ExternalResponse, IdOnly, Response, ResponseTypes},
};
use crate::library::security::{get_token, validate_token};
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_codegen::delete;

/**
 * @todo Create the method in nextcloud-rust-api
 */
#[delete("/nextcloud/user")]
pub fn sync_delete_nextcloud_user(
    req: HttpRequest,
    params: Json<IdOnly>,
) -> actix_web::Result<HttpResponse> {
    match get_token(req.headers()) {
        Ok(x) => {
            if validate_token(x.as_str()).unwrap() {
                let res = |id: String| -> Result<String, Box<dyn std::error::Error>> {
                    Ok(execute(defaults::NEXT, &["delete", "-i", id.as_str()])?)
                };

                Ok(match res(params.clone().id) {
                    Ok(x) => {
                        let x = ss!(ExternalResponse<bool>, x.as_str());
                        let x = Response::ok(200, ResponseTypes::Boolean(x.value.unwrap()));
                        response!(Ok, x)
                    }
                    Err(e) => response!(BadRequest, e.to_string()),
                    _ => response!(InternalServerError, defaults::ERROR_500),
                })
            } else {
                Ok(response!(Unauthorized, defaults::ERROR_401))
            }
        }
        Err(e) => Ok(response!(BadRequest, defaults::ERROR_400)),
    }
}
