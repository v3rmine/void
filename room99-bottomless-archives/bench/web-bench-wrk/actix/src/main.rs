use actix_web::{web, App, HttpServer, Responder};

async fn ping() -> impl Responder {
    "pong"
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/ping", web::get().to(ping));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let serv = HttpServer::new(move || App::new().configure(routes));
    serv.bind(
        &[
            "127.0.0.1:",
            &std::env::args()
                .collect::<Vec<String>>()
                .get(1)
                .unwrap_or(&String::from("8080")),
        ]
        .concat(),
    )?
    .run()
    .await
}
