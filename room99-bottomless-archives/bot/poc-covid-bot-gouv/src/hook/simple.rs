use super::util;
use hyper::{Body, Response};

use std::collections::HashMap;
use std::future::Future;
use std::sync::mpsc;

type Resp<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub use tokio::task::spawn as async_spawn;

// Todo => hook builder like hyper::Request::builder
#[allow(clippy::implicit_hasher)]
pub async fn hook<F>(
    uri: &str,
    headers: Option<HashMap<&str, &str>>,
    chan: Option<mpsc::Receiver<()>>,
    client: util::HyperClient,
    callback: impl Fn(Response<Body>) -> F,
) -> Resp<()>
where
    F: Future,
{
    let headers = match headers {
        Some(h) => h
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect::<HashMap<_, _>>(),
        None => HashMap::new(),
    };

    if let Some(chan) = chan {
        while chan.try_recv().is_err() {
            let query = util::get_uri(uri, headers.clone(), client.clone()).await?;
            callback(query).await;
        }
    } else {
        loop {
            let query = util::get_uri(uri, headers.clone(), client.clone()).await?;
            callback(query).await;
        }
    }
    Ok(())
}
