use std::{collections::HashMap, sync::Arc};

use pingora::{server::ShutdownWatch, services::background::BackgroundService};
use tokio::{
    sync::{mpsc::Receiver, oneshot, RwLock},
    time::Instant,
};
use tracing::{debug, info};

use super::backend::{BackendState, DockerServiceBackend, ProxyServiceBackend};

#[derive(Debug)]
pub(super) struct ServiceStarter {
    pub(super) services_starter: RwLock<Receiver<(String, oneshot::Sender<String>)>>,
    pub(super) services_state: Arc<RwLock<HashMap<String, Instant>>>,
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
                // TODO: get info + backend in DB
                let mut backend = DockerServiceBackend::new_backend().await.unwrap();

                if backend
                    .status("nginx")
                    .await
                    .is_ok_and(|s| s == BackendState::Started)
                {
                    debug!("service {required_service} already started");
                } else {
                    debug!("starting service {required_service}");
                    if backend.start("nginx").await.is_err() {
                        continue;
                    }
                }

                self.services_state
                    .write()
                    .await
                    .insert(required_service.clone(), Instant::now());

                // this should never panic because we just inserted the sender
                started.send("localhost:8080".to_string()).unwrap();
            }
        }

        info!("service starter shutting down");
    }
}
