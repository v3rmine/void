use hyper::client::HttpConnector;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, Request, Response};
use hyper_tls::HttpsConnector;

use std::collections::HashMap;
use std::sync::Arc;

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type HyperClient = Arc<Client<HttpsConnector<HttpConnector>, Body>>;

#[allow(clippy::implicit_hasher)]
pub async fn get_uri(
    uri: &str,
    headers: HashMap<String, String>,
    client: HyperClient,
) -> Resp<Response<Body>> {
    let mut req = Request::get(uri);
    append_headers(req.headers_mut().unwrap(), headers)?;
    let req = req.body(Body::empty())?;

    let resp = if uri.starts_with("http://") || uri.starts_with("https://") {
        client.request(req).await?
    } else {
        eprintln!("Unsupported protocol");
        std::process::exit(1);
    };
    Ok(resp)
}

#[allow(clippy::implicit_hasher)]
pub async fn post_uri(
    uri: &str,
    headers: HashMap<String, String>,
    body: &str,
    client: HyperClient,
) -> Resp<Response<Body>> {
    let mut req = Request::post(uri);

    append_headers(req.headers_mut().unwrap(), headers)?;
    let req = req.body(Body::from(hyper::body::Bytes::copy_from_slice(
        body.as_bytes(),
    )))?;

    let resp = if uri.starts_with("http://") || uri.starts_with("https://") {
        client.request(req).await?
    } else {
        eprintln!("Unsupported protocol");
        std::process::exit(1);
    };

    Ok(resp)
}

#[allow(clippy::implicit_hasher)]
fn append_headers(builder: &mut hyper::HeaderMap, headers: HashMap<String, String>) -> Resp<()> {
    for (k, v) in headers {
        builder.insert(
            HeaderName::from_lowercase(k.to_lowercase().as_bytes())?,
            HeaderValue::from_str(v.as_str())?,
        );
    }

    Ok(())
}
