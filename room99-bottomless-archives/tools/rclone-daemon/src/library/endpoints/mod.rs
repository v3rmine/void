#![allow(dead_code)]
pub use super::{structs, Response, ResponseTypes};
use actix_web::http::Method;
use actix_web::{HttpRequest, HttpResponse};

pub const PRIVATE: &[u8] = include_bytes!("../../../keys/daemon.prv");
pub const ENDPOINTS: &[u8] = include_bytes!("../../../keys/endpoints.pub");

fn d() {}

macro_rules! decrypt {
    ($text:expr) => {
        |text: String| -> Result<String, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::private_key_from_pem(
                crate::library::endpoints::PRIVATE,
            )?;
            Ok(String::from_utf8(ah_tools::reexports::decode(
                &ah_tools::security::private_decrypt(
                    &key,
                    serde_json::from_str::<ah_tools::security::Tomb>(
                        String::from_utf8(ah_tools::reexports::decode(&text)?)?.as_str(),
                    )?,
                )?
                .value,
            )?)
            .map_err(|_| "Error decrypting")?)
        }($text)
    };
}

macro_rules! response {
    ($code:ident, $json:expr) => {
        || -> Result<HttpResponse, Box<dyn std::error::Error>> {
            let key = ah_tools::reexports::rsa::Rsa::public_key_from_pem(
                crate::library::endpoints::ENDPOINTS,
            )?;
            let response = serde_json::to_string(&$json).unwrap();
            let response = ah_tools::security::public_encrypt(&key, response.as_bytes())?;
            let response = ah_tools::reexports::encode(&serde_json::to_string(&response).unwrap());
            let response = HttpResponse::$code()
                .content_type("application/json")
                .body(&response);
            Ok(response)
        }()
        .map_err(|_| ())?
    };
}

mod rclone_delete_conf;
pub use rclone_delete_conf::*;

mod rclone_get_conf;
pub use rclone_get_conf::*;

mod next_post_occ_restart;
pub use next_post_occ_restart::*;

mod next_post_occ_scan;
pub use next_post_occ_scan::*;

mod rclone_post_service_start;
pub use rclone_post_service_start::*;

mod rclone_post_service_stop;
pub use rclone_post_service_stop::*;

mod rclone_post_conf;
pub use rclone_post_conf::*;

mod generic_post_scan;
pub use generic_post_scan::*;

mod next_post_occ;
pub use next_post_occ::*;

mod next_delete_occ;
pub use next_delete_occ::*;

pub fn err404(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(match req.method().clone() {
        Method::GET => response!(NotFound, Response::err(404, "Not Found", "")),
        _ => response!(
            MethodNotAllowed,
            Response::err(405, "Method Not Allowed", "")
        ),
    })
}
