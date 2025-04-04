use std::{collections::HashMap, sync::Arc, time::Duration};

use pingora::{server::ShutdownWatch, services::background::BackgroundService};
use tokio::{
    sync::RwLock,
    time::{sleep, Instant},
};
use tracing::{debug, info};

use super::backend::{BackendState, DockerServiceBackend, ProxyServiceBackend};

#[derive(Debug)]
pub(super) struct ServiceStopper {
    pub(super) services_state: Arc<RwLock<HashMap<String, Instant>>>,
}
#[async_trait::async_trait]
impl BackgroundService for ServiceStopper {
    async fn start(&self, shutdown: ShutdownWatch) {
        info!("service stopper starting");

        while !*shutdown.borrow() {
            let mut services_to_stop: Vec<String> = Vec::new();
            self.services_state
                .write()
                .await
                .retain(|service, instant| {
                    let now = Instant::now();
                    if now - *instant > Duration::from_secs(30) {
                        services_to_stop.push(service.clone());

                        false
                    } else {
                        true
                    }
                });

            let mut backend = DockerServiceBackend::new_backend().await.unwrap();
            for service in services_to_stop {
                debug!("stopping service {service}");
                if backend
                    .status("nginx")
                    .await
                    .is_ok_and(|s| s == BackendState::Started)
                {
                    backend.stop("nginx").await.ok();
                }
            }

            sleep(Duration::from_secs(10)).await;
        }

        info!("service stopper shutting down");
    }
}
