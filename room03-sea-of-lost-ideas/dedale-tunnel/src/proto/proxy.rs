use std::{collections::HashMap, pin::Pin};

use bytes::BytesMut;
use pb::{proxy_server::*, *};
use request_stream::Protocol;
use tokio::{
    io::AsyncReadExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::mpsc,
};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tonic_health::server::HealthReporter;
use uuid::Uuid;

use crate::Origins;

pub mod pb {
    tonic::include_proto!("dedale.proxy");
}

pub struct ProxyServerService {
    health_reporter: HealthReporter,
    origins: Origins,
}

impl ProxyServerService {
    pub async fn new(health_reporter: HealthReporter, origins: Origins) -> ProxyServer<Self> {
        let mut proxy_server_service = Self {
            health_reporter,
            origins,
        };
        proxy_server_service.set_serving().await;

        ProxyServer::new(proxy_server_service)
    }

    async fn set_serving(&mut self) {
        self.health_reporter
            .set_serving::<ProxyServer<ProxyServerService>>()
            .await;
    }

    async fn set_not_serving(&mut self) {
        self.health_reporter
            .set_not_serving::<ProxyServer<ProxyServerService>>()
            .await;
    }
}

struct PendingRequest {
    uuid: Uuid,
    peer_ip: String,
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
}

#[tonic::async_trait]
impl Proxy for ProxyServerService {
    type ForwarderStream =
        Pin<Box<dyn Stream<Item = Result<RequestStream, Status>> + Send + 'static>>;
    #[tracing::instrument(skip(self))]
    async fn forwarder(
        &self,
        request: Request<Streaming<ResponseStream>>,
    ) -> Result<Response<Self::ForwarderStream>, Status> {
        let origins = self.origins.clone();
        let remote = request.remote_addr().unwrap();
        let (sender, mut receiver) = mpsc::channel::<TcpStream>(512);
        let mut pending_requests = HashMap::new() as HashMap<Uuid, PendingRequest>;

        tracing::info!(message = "connected to new origin", %remote);
        origins.write().await.insert(remote, sender);
        tracing::debug!(message = "origins", ?origins);

        // Handle origins responses
        let mut requests = request.into_inner();
        tokio::spawn(async move {
            loop {
                let req = requests.message().await;
                if req.is_ok_and(|i| i.is_none()) {
                    tracing::info!(message = "request closed by origin", %remote);
                    origins.write().await.remove(&remote);
                    tracing::debug!(message = "origins", ?origins);
                    break;
                }
            }
        });

        // Handle new requests
        tokio::spawn(async move {
            while let Some(tcp_stream) = receiver.recv().await {
                let peer_ip = tcp_stream.peer_addr().unwrap().to_string();
                let uuid = Uuid::now_v7();
                let (reader, writer) = tcp_stream.into_split();

                tracing::info!(message = "handling request", %uuid);
                pending_requests.insert(
                    uuid,
                    PendingRequest {
                        uuid,
                        peer_ip,
                        reader,
                        writer,
                    },
                );
            }
        });

        // Handle requests parts
        let mut pending_requests_reader = HashMap::new() as HashMap<Uuid, OwnedReadHalf>;
        let output = async_stream::try_stream! {
            let mut tcp_request = BytesMut::with_capacity(8192);
            // while tcp_reader.read_buf(&mut tcp_request).await.is_ok_and(|n| n > 0) {
            //     tracing::debug!(message = "read bytes", n = tcp_request.len());
            //
            //     yield RequestStream {
            //         request_uuid: request_uuid.to_string(),
            //         host: "localhost:9999".to_string(),
            //         client_ip: client_ip.clone(),
            //         protocol: Protocol::Http as i32,
            //         request: tcp_request.to_vec(),
            //     }
            // }
        };

        // Ok(Response::new(Box::pin(output) as Self::ForwarderStream))
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn auth(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        todo!()
    }

    type DiscoveryStream = Streaming<FrontendNodesStream>;
    #[tracing::instrument(skip(self))]
    async fn discovery(
        &self,
        request: Request<DiscoveryRequest>,
    ) -> Result<Response<Self::DiscoveryStream>, Status> {
        todo!()
    }
}
