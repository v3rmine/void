#![allow(dead_code)]
pub use super::{defaults, execute, execute_with_envs, structs};
use actix_web::{HttpRequest, HttpResponse};
use http::method::Method;

pub const PRIVATE: &[u8] = include_bytes!("../../../keys/endpoints.prv");
pub const DAEMON: &[u8] = include_bytes!("../../../keys/daemon.pub");
pub const HEADLESS: &[u8] = include_bytes!("../../../keys/headless.pub");
pub const NEXTCLOUD: &[u8] = include_bytes!("../../../keys/nextcloud.pub");

/* =.=.= Macros =.=.= */
/*
macro_rules! response {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::public_key_from_pem(crate::routes::EXTERNAL)?;
            let response = serde_json::to_string(&$json).unwrap();
            let response = ah_tools::security::public_encrypt(&key, response.as_bytes())?;
            let response = ah_tools::reexports::encode(&serde_json::to_string(&response).unwrap());
            let response = HttpResponse::$code()
                .content_type("application/json")
                .body(&response);
            Ok(response)
        }().map_err(|_| ())?
    };
}
*/

macro_rules! encrypt {
    ($key:expr, $e:expr) => {
        |k: &[u8], m: String| -> Result<String, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::public_key_from_pem(k)?;
            let r = ah_tools::security::public_encrypt(&key, m.as_bytes())?;
            let r = ah_tools::reexports::encode(&serde_json::to_string(&r).unwrap());
            Ok(r)
        }($key, $e)
        .map_err(|_| ())?
    };
}

macro_rules! decrypt {
    ($e:expr) => {
        |m: String| -> Result<String, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::private_key_from_pem(crate::routes::PRIVATE)?;
            let r = serde_json::from_str::<ah_tools::security::Tomb>(
                String::from_utf8(base64::decode(&m)?)?.as_str(),
            )?;
            let r = ah_tools::security::private_decrypt(&key, r)?;
            Ok(r.value)
        }($e)
        .map_err(|_| ())?
    };
}
macro_rules! response {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            Ok(HttpResponse::$code()
                .content_type("application/json")
                .body(&serde_json::to_string(&$json).unwrap()))
        }()
        .map_err(|_| ())?
    };
}
/* =.=.=.=.=.=.=.=.= */

/* =.=.= Routes =.=.= */
mod delete_nextcloud_user;
pub use delete_nextcloud_user::*;

mod get_nextcloud_cookies;
pub use get_nextcloud_cookies::*;

mod get_sa;
pub use get_sa::*;

mod post_nextcloud_user;
pub use post_nextcloud_user::*;

mod post_sa;
pub use post_sa::*;

mod post_session;
pub use post_session::*;

mod delete_session;
pub use delete_session::*;

pub fn err404(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(match req.method().clone() {
        Method::GET => response!(NotFound, defaults::ERROR_404),
        _ => response!(MethodNotAllowed, defaults::ERROR_405),
    })
}
/* =.=.=.=.=.=.=.=.= */
