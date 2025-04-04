mod library;
#[macro_use]
extern crate log;
#[macro_use]
extern crate pest_derive;

use actix_web::{middleware, web, App, HttpServer};
use badlog::init_from_env;
use get_if_addrs::get_if_addrs as get_addrs;
use library::{defaults, endpoints, is_env, set_var, var};
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();
    /* === ENVS === */
    is_env("LOG_LEVEL", &|_| (), &|env| set_var(env, "INFO"))?;
    init_from_env("LOG_LEVEL");
    is_env("DAEMON_NEXT", &|_| {}, &|env| match get_addrs() {
        Ok(x) => {
            let mut ints = x;
            ints.retain(|int| {
                let ip = int.addr.ip();
                ip.is_ipv4()
                    && ip != Ipv4Addr::from_str("127.0.0.1").unwrap()
                    && ip != Ipv4Addr::from_str("0.0.0.0").unwrap()
            });
            match ints.len() {
                1 => set_var(
                    env,
                    format!("{}:4242", ints[0].addr.ip().to_string()).as_str(),
                ),
                2 | _ => set_var(env, ":4242"),
            }
        }
        Err(_) => set_var(env, ":4242"),
    })?;
    info!("DAEMON_NEXT: {}", var("DAEMON_NEXT")?);
    /* === === === */

    /* === Websocket === */
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(endpoints::delete_conf)
            .service(endpoints::get_conf)
            .service(endpoints::post_occ_restart)
            .service(endpoints::post_occ_scan)
            .service(endpoints::post_occ_stop)
            .service(endpoints::post_conf)
            .service(endpoints::post_scan)
            .service(endpoints::post_occ)
            .service(endpoints::delete_occ)
            .default_service(web::resource("*").to_async(endpoints::err404))
    })
    .bind(var("DAEMON_NEXT")?)?
    .run()?;
    Ok(())
}
