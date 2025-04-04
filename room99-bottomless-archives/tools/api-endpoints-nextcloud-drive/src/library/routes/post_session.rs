use super::{
    defaults,
    structs::{NextCloudUser, Response},
};
use crate::library::security::{connect, create_token, get_token, is_connected, validate_token};
use crate::library::structures::ResponseTypes;
use actix_web::{web::Json, HttpRequest, HttpResponse};
use actix_web_codegen::post;

/**
 * @todo Generate SA Certs
 */
#[post("/session")]
pub fn sync_post_login(
    req: HttpRequest,
    params: Json<NextCloudUser>,
) -> actix_web::Result<HttpResponse> {
    Ok(match is_connected(params.clone().username.as_str()) {
        Ok(x) => {
            if x {
                match get_token(req.headers()) {
                    Ok(x) => match validate_token(x.as_str()) {
                        Ok(_) => response!(Ok, Response::ok(200, ResponseTypes::Base64(x))),
                        Err(_) => {
                            let token = create_token(params.clone().username).map_err(|_| ())?;
                            let token = ResponseTypes::Base64(token);
                            response!(Ok, Response::ok(200, token))
                        }
                    },
                    Err(_) => response!(BadRequest, defaults::ERROR_400),
                }
            } else {
                match connect(
                    params.clone().username.as_str(),
                    params.clone().password.as_str(),
                ) {
                    Ok(x) => {
                        if x {
                            let token = create_token(params.clone().username).map_err(|_| ())?;
                            let token = ResponseTypes::Base64(token);
                            response!(Ok, Response::ok(200, token))
                        } else {
                            response!(Unauthorized, defaults::ERROR_401)
                        }
                    }
                    Err(_) => response!(InternalServerError, defaults::ERROR_500),
                }
            }
        }
        Err(_) => response!(Unauthorized, defaults::ERROR_401),
    })
}
