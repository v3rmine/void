use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn ping(_req: &Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("pong")))
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/ping" => ping(&req).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("404 not found"))
            .expect("Cannot build 404 request")),
    }
}

#[tokio::main]
async fn main() {
    let default_port = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("8080"))
        .parse::<u16>()
        .expect("Cannot parse port");
    let addr = SocketAddr::from(([127, 0, 0, 1], default_port));
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
