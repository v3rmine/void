#[macro_use]
extern crate rocket;

#[get("/ping")]
fn hello() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> _ {
    let default_port = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("8080"))
        .parse::<u16>()
        .expect("Cannot parse port");
    let figment = rocket::Config::figment().merge(("port", default_port));

    rocket::custom(figment).mount("/", routes![hello])
}
