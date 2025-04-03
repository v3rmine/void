#![forbid(unsafe_code)]
// Source : https://fasterthanli.me/articles/the-curse-of-strong-typing#the-connect-trait-from-hyper

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use hyper::{
    client::{connect::Connection, HttpConnector},
    service::Service,
    Uri,
};
use hyper_tls::HttpsConnector;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone)]
pub struct PolyConnector {
    http: HttpConnector,
    https: HttpsConnector<HttpConnector>,
}

impl Default for PolyConnector {
    #[tracing::instrument(level = "trace")]
    fn default() -> Self {
        tracing::trace!("Creating a DockerConnector");

        Self {
            http: HttpConnector::new(),
            https: HttpsConnector::new(),
        }
    }
}

pub trait PolyConnection: AsyncRead + AsyncWrite + Connection {}
impl<T> PolyConnection for T where T: AsyncRead + AsyncWrite + Connection {}

impl Connection for Pin<Box<dyn PolyConnection + Send + 'static>> {
    fn connected(&self) -> hyper::client::connect::Connected {
        (**self).connected()
    }
}

impl Service<Uri> for PolyConnector {
    type Response = Pin<Box<dyn PolyConnection + Send + 'static>>;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        futures::ready!(self.http.poll_ready(cx))?;
        futures::ready!(self.https.poll_ready(cx))?;
        Ok(()).into()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn call(&mut self, req: Uri) -> Self::Future {
        tracing::trace!("Calling an Uri using DockerConnector");
        // keep it DRY (don't repeat yourself) with a macro...
        macro_rules! forward {
            ($target:expr) => {
                $target
                    .call(req)
                    .map_ok(|c| -> Self::Response { Box::pin(c) })
                    .map_err(|e| -> Self::Error { Box::new(e) })
                    .boxed()
            };
        }

        match req.scheme_str().unwrap_or_default() {
            "https" => self
                .https
                .call(req)
                .map_ok(|c| -> Self::Response { Box::pin(c) })
                .boxed(),
            _ => forward!(self.http),
        }
    }
}
