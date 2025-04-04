use super::{
    defaults, execute,
    structs::{ExternalResponse, NextCloudCoke, NextCloudUser, Response, ResponseTypes},
};
use crate::library::security::{get_token, validate_token};
use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_codegen::get;

#[get("/nextcloud/cookies")]
pub fn sync_get_nextcloud_cookies(
    req: HttpRequest,
    params: Json<NextCloudUser>,
) -> actix_web::Result<HttpResponse> {
    match get_token(req.headers()) {
        Ok(x) => {
            if validate_token(x.as_str()).unwrap() {
                let res = |(u, p): (String, String),
                           r: Option<String>|
                 -> Result<String, Box<dyn std::error::Error>> {
                    Ok(match r {
                        Some(x) => execute(
                            defaults::NEXT,
                            &[
                                "cookies",
                                "-u",
                                u.as_str(),
                                "-p",
                                p.as_str(),
                                "-e",
                                x.as_str(),
                            ],
                        )?,
                        None => execute(
                            defaults::NEXT,
                            &["cookies", "-u", u.as_str(), "-p", p.as_str()],
                        )?,
                    })
                };

                Ok(
                    match res(
                        (params.clone().username, params.clone().password),
                        params.clone().response_type,
                    ) {
                        Ok(x) => {
                            let x = ss!(ExternalResponse<NextCloudCoke>, x.as_str());
                            let x = Response::ok(200, ResponseTypes::Cookie(x.value.unwrap()));
                            response!(Ok, x)
                        }
                        Err(e) => response!(BadRequest, e.to_string()),
                        _ => response!(InternalServerError, defaults::ERROR_500),
                    },
                )
            } else {
                Ok(response!(Unauthorized, defaults::ERROR_401))
            }
        }
        Err(e) => Ok(response!(BadRequest, defaults::ERROR_400)),
    }
}
