use super::{
    defaults, execute_with_envs,
    structs::{ExternalResponse, NextCloudUser, Response, ResponseTypes},
};
use crate::library::security::{get_token, validate_token};
use actix_web::{web::Json, HttpRequest, HttpResponse};
use actix_web_codegen::post;
use ah_tools::is_env;

/**
 * @todo Create user in nextcloud
 */
#[post("/nextcloud/user")]
pub fn sync_post_nextcloud_user(
    req: HttpRequest,
    params: Json<NextCloudUser>,
) -> actix_web::Result<HttpResponse> {
    is_env("NC_ADMIN_USERNAME", &|_| {}, &|_| {
        panic!("Error NC_ADMIN_USERNAME not set");
    })
    .unwrap();
    is_env("NC_ADMIN_PASSWORD", &|_| {}, &|_| {
        panic!("Error NC_ADMIN_PASSWORD not set");
    })
    .unwrap();

    match get_token(req.headers()) {
        Ok(x) => {
            if validate_token(x.as_str()).unwrap() {
                let resp = execute_with_envs(
                    defaults::NEXT,
                    &[
                        "create",
                        "-u",
                        params.username.as_str(),
                        "-p",
                        params.password.as_str(),
                    ],
                    vec![
                        (
                            "NC_ADMIN_USERNAME",
                            crate::var("NC_ADMIN_USERNAME").unwrap().as_str(),
                        ),
                        (
                            "NC_ADMIN_PASSWORD",
                            crate::var("NC_ADMIN_PASSWORD").unwrap().as_str(),
                        ),
                    ],
                );
                Ok(match resp {
                    Ok(x) => {
                        let x: ExternalResponse<bool> = ss!(ExternalResponse<bool>, x.as_str());
                        if x.value.is_some() {
                            let x = Response::ok(200, ResponseTypes::Boolean(x.value.unwrap()));
                            response!(Ok, x)
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
        Err(e) => Ok(response!(BadRequest, defaults::ERROR_400)),
    }
}
