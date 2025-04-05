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
    server::{Server, ShutdownWatch},
    services::background::{background_service, BackgroundService},
    upstreams::peer::HttpPeer,
    ErrorType::InvalidHTTPHeader,
};
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot, RwLock,
    },
    time::{sleep, Instant},
};
use tracing::*;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
pub struct TestProxy {
    pub services_starter: Sender<(String, oneshot::Sender<()>)>,
    pub concurrent_req_count: AtomicU64,
}

#[derive(Debug)]
pub struct ProxyCtx {
    pub host: String,
    pub tx_service_started: Option<oneshot::Sender<()>>,
    pub rx_service_started: Option<oneshot::Receiver<()>>,
}

#[async_trait::async_trait]
impl ProxyHttp for TestProxy {
    type CTX = ProxyCtx;

    fn new_ctx(&self) -> Self::CTX {
        let (tx_service_started, rx_service_started) = oneshot::channel();
        Self::CTX {
            host: String::new(),
            tx_service_started: Some(tx_service_started),
            rx_service_started: Some(rx_service_started),
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

        let host = session
            .get_header("host")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| pingora::Error::explain(InvalidHTTPHeader, "no host header detected"))?
            .to_string();
        ctx.host = host.clone();
        // this function will run only once after initialization
        // so we can safely take and unwrap the sender
        self.services_starter
            .send((host, ctx.tx_service_started.take().unwrap()))
            .await
            .unwrap();

        Ok(false)
    }

    #[tracing::instrument(skip_all)]
    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let host = ctx.host.clone();

        debug!("waiting for {host} to be ready");
        // this function will run only once after initialization
        // so we can safely take and unwrap the receiver
        ctx.rx_service_started.take().unwrap().await.unwrap();
        debug!("done waiting for {host} to be ready");

        let peer = Box::new(HttpPeer::new(
            format!("{host}:80")
                .to_socket_addrs()
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

#[derive(Debug)]
struct ServiceStarter {
    services_starter: RwLock<Receiver<(String, oneshot::Sender<()>)>>,
    services_state: Arc<RwLock<HashMap<String, Instant>>>,
}

#[async_trait::async_trait]
impl BackgroundService for ServiceStarter {
    async fn start(&self, shutdown: ShutdownWatch) {
        info!("service starter starting");

        while !*shutdown.borrow() {
            while let Some((required_service, started)) =
                self.services_starter.write().await.recv().await
            {
                debug!("got request to start service {required_service}");

                let mut services = self.services_state.write().await;
                if services.contains_key(&required_service) {
                    debug!("service {required_service} already started");
                } else {
                    debug!("starting service {required_service}");
                }
                services.insert(required_service.clone(), Instant::now());
                drop(services);

                // this should never panic because we just inserted the sender
                started.send(()).unwrap();
            }
        }

        info!("service starter shutting down");
    }
}

#[derive(Debug)]
struct ServiceStopper {
    services_state: Arc<RwLock<HashMap<String, Instant>>>,
}
#[async_trait::async_trait]
impl BackgroundService for ServiceStopper {
    async fn start(&self, shutdown: ShutdownWatch) {
        info!("service stopper starting");

        while !*shutdown.borrow() {
            self.services_state
                .write()
                .await
                .retain(|service, instant| {
                    let now = Instant::now();
                    if now - *instant > Duration::from_secs(30) {
                        debug!("stopping service {service}");
                        false
                    } else {
                        true
                    }
                });
            sleep(Duration::from_secs(10)).await;
        }

        info!("service stopper shutting down");
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

    let mut server = Server::new(None)?;
    server.bootstrap();

    let (tx_need_service, rx_need_service) = channel(100);
    let services_state = Arc::new(RwLock::new(HashMap::new()));

    let mut proxy = http_proxy_service(
        &server.configuration,
        TestProxy {
            services_starter: tx_need_service,
            concurrent_req_count: AtomicU64::new(0),
        },
    );
    proxy.add_tcp("127.0.0.1:8080");
    info!("listening on 127.0.0.1:8080");

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

    /*let cli = Cli::parse();

    match cli.command {
        Commands::Launch { path } => {
            let config_path = PathBuf::from_str(&path)?.join("fly.toml");
            if !config_path.exists() {
                return Err(eyre!("Config file not found: {}", config_path.display()));
            }

            let config: Config = toml::from_str(&read_to_string(config_path)?)?;
            dbg!(config);
        }
    }*/

    Ok(())
}
