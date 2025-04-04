use super::{
    defaults, execute,
    structs::{CreateSA, DriveResponse, Response, ResponseTypes, ServiceAccount},
};
use crate::library::security::{get_token, validate_token};
use actix_web::{web::Json, HttpRequest, HttpResponse};
use actix_web_codegen::post;

/**
 * @todo Generate SA Certs
 */
#[post("/sa")]
pub fn sync_post_sa(req: HttpRequest, params: Json<CreateSA>) -> actix_web::Result<HttpResponse> {
    match get_token(req.headers()) {
        Ok(x) => {
            match validate_token(x.as_str()) {
                Ok(check) => {
                    if check {
                        let encoding = match params.clone().response_type {
                            Some(x) => x,
                            None => "plaintext".to_owned(),
                        };
                        let username: String = encrypt!(super::HEADLESS, params.clone().username);
                        let password: String = encrypt!(super::HEADLESS, params.clone().password);
                        let resp = execute(
                            defaults::CERTIFICATOR,
                            &[
                                "-u",
                                format!("{}", username.as_str()).as_str(),
                                "-p",
                                format!("{}", password.as_str()).as_str(),
                                "-n",
                                params.project_name.as_str(),
                                "-i",
                                params.sa_id.as_str(),
                                "-e",
                                encoding.as_str(), //"-h"
                                "-v",
                                "error",
                            ],
                        );
                        Ok(match resp {
                            Ok(x) => {
                                let x: String = decrypt!(x);
                                let x: DriveResponse = ss!(DriveResponse, x.as_str());
                                if x.json.is_some() {
                                    if encoding == "base64" && x.p12_as_base64.is_some() {
                                        let result = Response::ok(
                                            200,
                                            ResponseTypes::SABase64(ServiceAccount {
                                                json: x.json,
                                                p12: x.p12_as_base64,
                                            }),
                                        );
                                        response!(Ok, result)
                                    } else if x.p12_as_bytes.is_some() {
                                        let result = Response::ok(
                                            200,
                                            ResponseTypes::SABinary(ServiceAccount {
                                                json: x.json,
                                                p12: x.p12_as_bytes,
                                            }),
                                        );
                                        response!(Ok, result)
                                    } else {
                                        response!(InternalServerError, defaults::ERROR_500)
                                    }
                                } else {
                                    response!(InternalServerError, defaults::ERROR_500)
                                }
                            }
                            Err(e) => response!(InternalServerError, defaults::ERROR_500),
                        })
                    } else {
                        Ok(response!(Unauthorized, defaults::ERROR_401))
                    }
                }
                Err(_) => Ok(response!(Unauthorized, defaults::ERROR_401)),
            }
        }
        Err(e) => Ok(response!(BadRequest, defaults::ERROR_400)),
    }
}
