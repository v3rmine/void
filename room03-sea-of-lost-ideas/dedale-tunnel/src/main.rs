// TODO: Remove this before release
#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, io, net::SocketAddr, sync::Arc};

use clap::Parser;
use proto::{proxy::ProxyServerService, report::ReportServerService};
use tokio::{
    net::{TcpListener, TcpStream},
    pin,
    sync::{mpsc::Sender, RwLock},
};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Server;
use tracing_subscriber::EnvFilter;

mod proto;
mod utils;

#[derive(clap::Parser)]
struct App {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Tunnel,
    Proxy,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("LOG_LEVEL"))
        .init();
    let app = App::parse();

    match app.subcommand {
        Command::Tunnel => tunnel().await,
        Command::Proxy => proxy().await,
    }
}

async fn tunnel() -> color_eyre::Result<()> {
    Ok(())
}

fn bind_and_accept(addr: SocketAddr) -> impl Stream<Item = io::Result<TcpStream>> {
    async_stream::try_stream! {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            tracing::info!(message = "receiving requests", %addr);
            yield stream;
        }
    }
}

type Origins = Arc<RwLock<HashMap<SocketAddr, Sender<TcpStream>>>>;

async fn proxy() -> color_eyre::Result<()> {
    let grpc_addr = "0.0.0.0:50031".parse()?;
    let origins = Arc::new(RwLock::new(HashMap::new())) as Origins;

    let origins_to_request = origins.clone();
    tokio::spawn(async {
        let origins = origins_to_request;
        let proxy_addr = "0.0.0.0:8080".parse().unwrap();
        let requests = bind_and_accept(proxy_addr);

        pin!(requests);

        while let Some(request) = requests.next().await {
            let origins = { origins.read().await };
            let request = request.unwrap();

            tracing::info!(message = "received request", ?request);
            if let Some((_socket, origin)) = origins.iter().next() {
                origin.send(request).await.unwrap();
            }
        }
    });

    // Allow to watch status of the services
    let (health_reporter, health_service) = tonic_health::server::health_reporter();

    let proxy_server_service =
        ProxyServerService::new(health_reporter.clone(), origins.clone()).await;
    let report_server_service = ReportServerService::new(health_reporter.clone()).await;

    tracing::info!(message = "starting GRPC server", %grpc_addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("dedale-proxy"))
        .add_service(health_service)
        .add_service(proxy_server_service)
        .add_service(report_server_service)
        .serve(grpc_addr)
        .await?;

    Ok(())
}
