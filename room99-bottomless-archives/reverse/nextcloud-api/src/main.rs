mod app;
mod library;
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

use crate::library::serde_is_valid_and_contain;
use badlog::init_from_env;
use library::{
    create_user, is_env, private_decrypt, public_encrypt, set_var, var, ConvertTo, GenericValue,
    QueryCreateUser, QueryGetToken, Response, ResponseTypes, Tomb, UserCookies,
};

const ENDPOINT: &[u8] = include_bytes!("../keys/endpoints.pub");
const PRIVATE: &[u8] = include_bytes!("../keys/nextcloud.prv");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();
    let endpoint_pub = openssl::rsa::Rsa::public_key_from_pem(ENDPOINT)?;
    let self_private = openssl::rsa::Rsa::private_key_from_pem(PRIVATE)?;

    let mut app_base = app::build_cli();
    let app = app_base.clone().get_matches();
    let (sub, command) = match app.subcommand_matches("create") {
        Some(_) => (app.subcommand_matches("create"), "create_user"),
        None => match app.subcommand_matches("cookies") {
            Some(_) => (app.subcommand_matches("cookies"), "get_token"),
            None => {
                app_base.print_help()?;
                println!("\n");
                std::process::exit(-1);
            }
        },
    };
    let sub = sub.unwrap();

    match sub.value_of("log-level") {
        Some(x) => set_var("LOG_LEVEL", x.to_uppercase()),
        None => {
            is_env("LOG_LEVEL", &|_| (), &|env| set_var(env, "INFO"));
        }
    }
    init_from_env("LOG_LEVEL");

    match sub.value_of("timeout") {
        Some(x) => set_var("HEADLESS_TIMEOUT", x),
        None => {
            is_env("HEADLESS_TIMEOUT", &|_| {}, &|env| set_var(env, "3000"));
        }
    }
    info!("Headless timeout: {}", var("HEADLESS_TIMEOUT")?);

    is_env("BASE_NC_URL", &|_| {}, &|env| {
        set_var(env, "https://files.hume.cloud")
    });

    let username_uncrypt = String::from_utf8(base64::decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(
                String::from_utf8(base64::decode(&sub.value_of("username").unwrap())?)?.as_str(),
            )?,
        )?
        .value,
    )?)?;
    let password_uncrypt = String::from_utf8(base64::decode(
        &private_decrypt(
            &self_private,
            serde_json::from_str::<Tomb>(
                String::from_utf8(base64::decode(&sub.value_of("password").unwrap())?)?.as_str(),
            )?,
        )?
        .value,
    )?)?;

    let text = &match command {
        "create_user" => format!(
            r#"{{"command":"{}","username":"{}","password":"{}"}}"#,
            command, username_uncrypt, password_uncrypt
        ),
        "get_token" => format!(
            r#"{{"command":"{}","username":"{}","password":"{}","response_type":"{}"}}"#,
            command,
            username_uncrypt,
            password_uncrypt,
            sub.value_of("response-type").unwrap()
        ),
        _ => std::process::exit(-1),
    }[..];

    let result = if serde_is_valid_and_contain::<QueryCreateUser>(text, "command", "create_user") {
        let msg = serde_json::from_str::<QueryCreateUser>(text).unwrap();
        is_env(
            "NC_ADMIN_USERNAME",
            &|env| info!("{} is set", env),
            &|env| {
                info!("{} is not set", env);
                let response = public_encrypt(
                    &endpoint_pub,
                    serde_json::to_string(&Response::<GenericValue<ResponseTypes>> {
                        status_code: 503,
                        error_msg: Some(
                            "The server is unavailable to handle this request right now",
                        ),
                        error_details: Some("Error! Missing some env variables"),
                        value: None,
                    })
                    .unwrap()
                    .as_bytes(),
                )
                .unwrap();
                let response = base64::encode(&serde_json::to_string(&response).unwrap());
                println!("{}", response);
                std::process::exit(-1);
            },
        );
        is_env(
            "NC_ADMIN_PASSWORD",
            &|env| info!("{} is set", env),
            &|env| {
                info!("{} is not set", env);
                let response = public_encrypt(
                    &endpoint_pub,
                    serde_json::to_string(&Response::<GenericValue<ResponseTypes>> {
                        status_code: 503,
                        error_msg: Some(
                            "The server is unavailable to handle this request right now",
                        ),
                        error_details: Some("Error! Missing some env variables"),
                        value: None,
                    })
                    .unwrap()
                    .as_bytes(),
                )
                .unwrap();
                let response = base64::encode(&serde_json::to_string(&response).unwrap());
                println!("{}", response);
                std::process::exit(-1);
            },
        );
        match create_user(msg) {
            Ok(_) => {
                let val: GenericValue<ResponseTypes> = Some(ResponseTypes::Boolean(true));
                Response {
                    status_code: 200,
                    error_msg: None,
                    error_details: None,
                    value: val,
                }
            }
            Err(_) => {
                let val: GenericValue<ResponseTypes> = Some(ResponseTypes::Boolean(false));
                Response {
                    status_code: 500,
                    error_msg: Some("Internal Server Error"),
                    error_details: Some("Error! Cannot create the user"),
                    value: val,
                }
            }
        }
    } else if serde_is_valid_and_contain::<QueryGetToken>(text, "command", "get_token") {
        let msg = serde_json::from_str::<QueryGetToken>(text).unwrap();
        #[allow(unused_assignments)]
        let mut resp = (String::new(), String::new(), String::new());
        let val_converter = msg.value_type;
        match library::get_tokens(msg) {
            Ok(_resp) => {
                resp = _resp;
                let val: GenericValue<ResponseTypes> = Some(ResponseTypes::Cookies(UserCookies {
                    nc_session_id: val_converter.convert_to(resp.0.as_str()),
                    nc_token: val_converter.convert_to(resp.1.as_str()),
                    nc_username: val_converter.convert_to(resp.2.as_str()),
                }));
                Response {
                    status_code: 200,
                    error_msg: None,
                    error_details: None,
                    value: val,
                }
            }
            Err(e) => {
                warn!("{}", e);
                Response {
                    status_code: 500,
                    error_msg: Some("Internal Server Error"),
                    error_details: Some("Error! Cannot get the tokens"),
                    value: None,
                }
            }
        }
    } else {
        Response {
            status_code: 500,
            error_msg: Some("Internal Server Error"),
            error_details: Some("Error! Cannot get the tokens"),
            value: None,
        }
    };
    let response = public_encrypt(
        &endpoint_pub,
        serde_json::to_string(&result).unwrap().as_bytes(),
    )?;
    let response = base64::encode(&serde_json::to_string(&response)?);
    println!("{}", response);
    std::process::exit(0)
}
