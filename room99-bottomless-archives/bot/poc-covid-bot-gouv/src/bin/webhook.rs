#![allow(deprecated)]
use std::sync::Arc;
use std::thread;

use gouv_rs::{async_spawn, hook, util};
use hyper::{Body, Response};

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Resp<()> {
    let client = Arc::new(hyper::Client::builder().build(hyper_tls::HttpsConnector::new()));
    let (tx, rx) = std::sync::mpsc::channel();
    let x = async_spawn(hook("http://127.0.0.1:3000/", None, Some(rx), process_body));
    thread::sleep_ms(10000);
    tx.send(())?;
    x.await??;
    Ok(())
}

async fn process_body(body: Response<Body>) -> Resp<()> {
    let body = hyper::body::to_bytes(body.into_body()).await?;
    let body = String::from_utf8(body.to_vec()).unwrap();

    let hashbody = util::sha256(&body);
    println!("{}", hashbody);
    let oldhash = util::read_file("./log.txt");
    println!("{}", oldhash);

    if oldhash != hashbody {
        println!("Content updated!");
        util::write_file("./log.txt", &hashbody);
    } else {
        println!("Content unchanged!");
    }

    thread::sleep_ms(1000);
    Ok(())
}
