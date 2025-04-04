#![feature(const_fn)]
#![feature(const_vec_new)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate ah_tools;
#[macro_use]
extern crate lazy_static;

mod library;

use actix_web::{middleware, web, App, HttpServer};
use ah_tools::security::{thread_rng, Rng};
use ah_tools::simple_db::SimpleDB;
use ah_tools::{is_env, set_var, var};
use badlog::init_from_env;
use library::security::{RegisteredUser, User};
use library::{defaults, routes};
use std::sync::{Arc, Mutex, MutexGuard};

fn get_registered() -> Vec<RegisteredUser> {
    let x = SimpleDB::init("user_db").unwrap();
    x.add(
        "debug@localhost",
        "$2y$12$IgSmNUX0fm.9Ey.tvVPiNOUrbBO0cTkvQUpMbKJdzJZQ9jt4g/NqW",
    )
    .unwrap();
    x.get_all()
        .unwrap()
        .iter()
        .map(|(x, y)| RegisteredUser {
            email: x.to_owned(),
            password: y.to_owned(),
        })
        .collect::<Vec<RegisteredUser>>()
}
lazy_static! {
    static ref LOGGED: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
    static ref USERS: Arc<Mutex<Vec<RegisteredUser>>> = Arc::new(Mutex::new(Vec::new()));
    static ref KEY: [u8; 16] = thread_rng().gen::<[u8; 16]>();
    static ref IV: [u8; 16] = thread_rng().gen::<[u8; 16]>();
}

fn main() -> defaults::ErrorChainResult<()> {
    let mut locked: MutexGuard<Vec<RegisteredUser>> = USERS.lock().unwrap();
    for i in get_registered() {
        locked.push(i);
    }
    drop(locked);
    /* =.=.= Setup env variables =.=.= */
    is_env("LOG_LEVEL", &|_| {}, &|env| {
        set_var(env, defaults::LOG_LEVEL)
    })?;
    init_from_env("LOG_LEVEL");
    is_env("DEFAULT_ADDR", &|_| {}, &|env| {
        set_var(env, defaults::ADDR);
    })?;
    info!("Listening on {} (DEFAULT_ADDR)", var("DEFAULT_ADDR")?);
    /* =.=.=.=.=.=.=.=.= */

    /* =.=.= Webserver init =.=.= */
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::sync_get_sa)
            .service(routes::sync_post_sa)
            .service(routes::sync_post_nextcloud_user)
            .service(routes::sync_delete_nextcloud_user)
            .service(routes::sync_get_nextcloud_cookies)
            .service(routes::sync_post_login)
            .service(routes::sync_post_logout)
            .default_service(web::resource("*").to_async(routes::err404))
    })
    .bind(var("DEFAULT_ADDR")?)?
    .run()?;
    /* =.=.=.=.=.=.=.=.= */

    Ok(())
}
