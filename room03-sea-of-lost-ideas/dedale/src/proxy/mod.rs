use std::{
    collections::HashMap,
    net::ToSocketAddrs,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use pingora::{
    self,
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::Server,
    services::background::background_service,
    upstreams::peer::HttpPeer,
    ErrorType::InvalidHTTPHeader,
};
use service_starter::ServiceStarter;
use service_stopper::ServiceStopper;
use tokio::{
    sync::{
        mpsc::{channel, Sender},
        oneshot, RwLock,
    },
    time::sleep,
};
use tracing::{debug, info, warn};

pub mod backend;
mod service_starter;
mod service_stopper;

const MAX_RETRY_COUNT: u16 = 3;

#[derive(Debug)]
pub(super) struct Proxy {
    services_starter: Sender<(String, oneshot::Sender<String>)>,
    concurrent_req_count: AtomicU64,
    default_service: Option<String>,
}

impl Proxy {
    pub fn run(listen_url: &str, default_service: Option<&str>) -> pingora::Result<()> {
        let mut server = Server::new(None)?;
        server.bootstrap();

        let (tx_need_service, rx_need_service) = channel(1024);
        let services_state = Arc::new(RwLock::new(HashMap::new()));

        let mut proxy = http_proxy_service(
            &server.configuration,
            Self {
                services_starter: tx_need_service,
                concurrent_req_count: AtomicU64::new(0),
                default_service: default_service.map(str::to_string),
            },
        );
        proxy.add_tcp(listen_url);
        info!("listening on {listen_url}");

        let start_service_handler = background_service(
            "service-starter",
            ServiceStarter {
                services_starter: RwLock::new(rx_need_service),
                services_state: services_state.clone(),
            },
        );
        let stop_service_handler =
            background_service("service-stopper", ServiceStopper { services_state });

        server.add_service(proxy);
        server.add_service(start_service_handler);
        server.add_service(stop_service_handler);
        
        server.run_forever();
    }
}

#[derive(Debug)]
pub(super) struct ProxyCtx {
    host: Option<String>,
    tx_service_started: Option<oneshot::Sender<String>>,
    rx_service_started: Option<oneshot::Receiver<String>>,
    retry_count: u16,
}

#[async_trait::async_trait]
impl ProxyHttp for Proxy {
    type CTX = ProxyCtx;

    fn new_ctx(&self) -> Self::CTX {
        let (tx_service_started, rx_service_started) = oneshot::channel();
        Self::CTX {
            host: None,
            tx_service_started: Some(tx_service_started),
            rx_service_started: Some(rx_service_started),
            retry_count: 0,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        debug!(
            "got a request {}",
            self.concurrent_req_count.fetch_add(1, Ordering::SeqCst)
        );

        if let Some(host) = session.get_header("host").and_then(|h| {
            h.to_str()
                .ok()
                .and_then(|h| if false { None } else { Some(h) })
        }) {
            let host = host.to_string();
            ctx.host = Some(host.clone());
            // this function will run only once after initialization
            // so we can safely take and unwrap the sender
            self.services_starter
                .send((host, ctx.tx_service_started.take().unwrap()))
                .await
                .unwrap();
        }

        Ok(false)
    }

    #[tracing::instrument(skip_all)]
    fn error_while_proxy(
        &self,
        peer: &HttpPeer,
        session: &mut Session,
        e: Box<pingora::Error>,
        ctx: &mut Self::CTX,
        client_reused: bool,
    ) -> Box<pingora::Error> {
        let mut e = e.more_context(format!("Peer: {peer}"));

        if ctx.retry_count < MAX_RETRY_COUNT
            && e.cause
                .as_ref()
                .is_some_and(|e| e.to_string().contains("Connection reset by peer"))
        {
            ctx.retry_count += 1;
            info!("retrying connection (count: {})", ctx.retry_count);
            e.set_retry(true);
        }

        // only reused client connections where retry buffer is not truncated
        e.retry
            .decide_reuse(client_reused && !session.as_ref().retry_buffer_truncated());

        e
    }

    #[tracing::instrument(skip_all)]
    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let host = if ctx.retry_count > 0 {
            // We sleep here to give some time before retrying
            // We can sleep because we use the tokio version
            sleep(Duration::from_millis(10)).await;
            debug!("retrying with same host");
            ctx.host.clone().unwrap()
        } else if let Some(ref host) = ctx.host {
            debug!("waiting for {host} to be ready");
            // this function will run only once after initialization
            // so we can safely take and unwrap the receiver
            // TODO: Handle timeout
            let host = ctx.rx_service_started.take().unwrap().await.unwrap();
            debug!("done waiting for {host} to be ready");

            // Update target host in ctx
            ctx.host = Some(host.clone());

            host
        } else if let Some(ref host) = self.default_service {
            // Update target host in ctx
            ctx.host = Some(host.clone());

            host.clone()
        } else {
            return Err(pingora::Error::explain(
                InvalidHTTPHeader,
                "no host header and no default service",
            ));
        };

        let peer = Box::new(HttpPeer::new(
            host.to_socket_addrs()
                .map(|mut s| s.next())
                .ok()
                .flatten()
                .ok_or_else(|| pingora::Error::explain(InvalidHTTPHeader, "invalid host"))?,
            false,
            host,
        ));

        Ok(peer)
    }

    #[tracing::instrument(skip_all)]
    async fn logging(
        &self,
        _session: &mut Session,
        _e: Option<&pingora::Error>,
        _ctx: &mut Self::CTX,
    ) where
        Self::CTX: Send + Sync,
    {
        debug!(
            "end of request {}",
            self.concurrent_req_count.fetch_sub(1, Ordering::SeqCst)
        );
    }
}
