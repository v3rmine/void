mod app;
mod library;
#[macro_use]
extern crate log;

use ah_tools::is_env;
use ah_tools::reexports::rsa::Rsa;
use ah_tools::security::{private_decrypt, public_encrypt, Tomb};
use badlog;
use base64::{decode, encode};
use library::{sa_setup, Params, Response};
use std::env::{set_var, var};

const ENDPOINT: &[u8] = include_bytes!("../keys/endpoints.pub");
const PRIVATE: &[u8] = include_bytes!("../keys/headless.prv");

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let app = app::build_cli().get_matches();
    let endpoint_pub = Rsa::public_key_from_pem(ENDPOINT)?;
    let self_private = Rsa::private_key_from_pem(PRIVATE)?;

    match app.value_of("log-level") {
        Some(x) => set_var("LOG_LEVEL", x.to_uppercase()),
        None => {
            is_env("LOG_LEVEL", &|_| {}, &|env| set_var(env, "WARN"))?;
        }
    }
    badlog::init_from_env("LOG_LEVEL");

    match app.value_of("timeout") {
        Some(x) => set_var("HEADLESS_TIMEOUT", x),
        None => {
            is_env("HEADLESS_TIMEOUT", &|_| {}, &|env| set_var(env, "3000"))?;
        }
    }
    info!("Headless timeout: {}", var("HEADLESS_TIMEOUT").unwrap());
    let username_uncrypt = String::from_utf8(decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(
                String::from_utf8(decode(&app.value_of("username").unwrap())?)?.as_str(),
            )?,
        )?
        .value,
    )?)?;
    let password_uncrypt = String::from_utf8(decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(
                String::from_utf8(decode(&app.value_of("password").unwrap())?)?.as_str(),
            )?,
        )?
        .value,
    )?)?;

    let msg = Params {
        username: username_uncrypt.as_str(),
        password: password_uncrypt.as_str(),
        project_name: app.value_of("project-name").unwrap(),
        sa_id: app.value_of("sa-id").unwrap(),
        response_type: app.value_of("encoding").unwrap(),
        timeout: app.value_of("sleeptime").and_then(|x| {
            Some(
                x.parse::<u64>()
                    .expect("Error while parsing the sleeptime!"),
            )
        }),
        headless: app.is_present("headless"),
    };
    let mut _json = String::new();
    let mut _bytes = Vec::<u8>::new();
    let mut _base64 = String::new();
    let response = match sa_setup(msg) {
        Some((x, y)) => {
            _json = x;
            _bytes = y;
            match msg.response_type {
                "base64" => {
                    _base64 = encode(_bytes.as_slice());
                    _json = serde_json::to_string(
                        &serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                            _json.as_str(),
                        )
                        .unwrap(),
                    )
                    .unwrap();
                    _json = encode(_json.as_bytes());
                    Response {
                        code: 200,
                        error: None,
                        p12_as_bytes: None,
                        p12_as_base64: Some(_base64.as_str()),
                        json: Some(_json.as_str()),
                    }
                }
                _ => {
                    _json = serde_json::to_string(
                        &serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                            _json.as_str(),
                        )
                        .unwrap(),
                    )
                    .unwrap();
                    Response {
                        code: 200,
                        error: None,
                        p12_as_bytes: Some(_bytes.as_slice()),
                        p12_as_base64: None,
                        json: Some(_json.as_str()),
                    }
                }
            }
        }
        None => Response {
            code: 500,
            error: Some("Internal Server Error"),
            p12_as_bytes: None,
            p12_as_base64: None,
            json: None,
        },
    };
    let response = public_encrypt(
        &endpoint_pub,
        serde_json::to_string(&response).unwrap().as_bytes(),
    )?;
    let response = serde_json::to_string(&response)?;
    println!("{}", encode(&response));
    Ok(())
}
