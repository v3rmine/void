use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let resp = fs::read_to_string("./index.html").unwrap_or_else(|_| "".to_owned());
    Ok(Response::new(resp.into()))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
