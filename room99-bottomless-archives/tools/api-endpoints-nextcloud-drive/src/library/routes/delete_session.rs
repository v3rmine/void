use super::{
    defaults,
    structs::{Response, ResponseTypes},
};
use crate::library::security::{disconnect, get_token, mail_from_token, validate_token};
use actix_web::{HttpRequest, HttpResponse};
use actix_web_codegen::delete;

/**
 * @todo Generate SA Certs
 */
#[delete("/session")]
pub fn sync_post_logout(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(match get_token(req.headers()) {
        Ok(x) => match validate_token(x.clone().as_str()) {
            Ok(v) => {
                if v {
                    match mail_from_token(x.clone()) {
                        Ok(x) => match disconnect(x.as_str()) {
                            Ok(x) => {
                                if x {
                                    response!(
                                        Ok,
                                        Response::ok(
                                            200,
                                            ResponseTypes::PlainText("disconnecting".to_owned())
                                        )
                                    )
                                } else {
                                    response!(InternalServerError, defaults::ERROR_403)
                                }
                            }
                            Err(_) => response!(InternalServerError, defaults::ERROR_500),
                        },
                        Err(_) => response!(InternalServerError, defaults::ERROR_500),
                    }
                } else {
                    response!(Unauthorized, defaults::ERROR_401)
                }
            }
            Err(_) => response!(Unauthorized, defaults::ERROR_401),
        },
        Err(_) => response!(BadRequest, defaults::ERROR_400),
    })
}
