use bollard::{
    container::{StartContainerOptions, StopContainerOptions},
    secret::ContainerStateStatusEnum,
    Docker,
};
use pingora::ErrorType::InternalError;

use super::{BackendState, ProxyServiceBackend};

pub struct DockerServiceBackend {
    docker: Docker,
}

impl ProxyServiceBackend for DockerServiceBackend {
    const IDENT: &'static str = "docker";

    async fn new_backend() -> pingora::Result<Self> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults()
                .map_err(|e| pingora::Error::explain(InternalError, e.to_string()))?,
        })
    }

    async fn status(&mut self, service: &str) -> pingora::Result<BackendState> {
        Ok(self
            .docker
            .inspect_container(service, None)
            .await
            .map_or_else(
                |_| BackendState::NotFound,
                |i| {
                    i.state
                        .map_or(BackendState::NotFound, |state| match state.status {
                            Some(ContainerStateStatusEnum::RUNNING) => BackendState::Started,
                            Some(
                                ContainerStateStatusEnum::CREATED
                                | ContainerStateStatusEnum::PAUSED,
                            ) => BackendState::Stopped,
                            _ => BackendState::NotFound,
                        })
                },
            ))
    }

    async fn start(&mut self, service: &str) -> pingora::Result<String> {
        self.docker
            .start_container(service, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| pingora::Error::explain(InternalError, e.to_string()))?;
        Ok(service.to_string())
    }

    async fn stop(&mut self, service: &str) -> pingora::Result<String> {
        self.docker
            .stop_container(service, None::<StopContainerOptions>)
            .await
            .map_err(|e| pingora::Error::explain(InternalError, e.to_string()))?;
        Ok(service.to_string())
    }
}
