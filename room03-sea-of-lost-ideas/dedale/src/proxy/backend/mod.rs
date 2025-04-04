#[cfg(feature = "backend_docker")]
mod docker;
#[cfg(feature = "backend_docker")]
pub(super) use docker::*;
use tracing::warn;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum BackendState {
    Started,
    Stopped,
    NotFound,
}

pub(super) trait ProxyServiceBackend {
    const IDENT: &'static str;

    async fn new_backend() -> pingora::Result<Self>
    where
        Self: Sized;

    /// Should return the state of the backend
    #[allow(clippy::unused_async)]
    async fn status(&mut self, service: &str) -> pingora::Result<BackendState> {
        warn!(
            "get status of {service} in {} status is not implemented",
            Self::IDENT
        );
        Ok(BackendState::NotFound)
    }
    /// Should start the service
    /// must be callable multiple times without error
    #[allow(clippy::unused_async)]
    async fn start(&mut self, service: &str) -> pingora::Result<String> {
        todo!("start {service} of {} is not implemented", Self::IDENT);
    }
    /// Should stop the service
    /// must be callable multiple times without error
    #[allow(clippy::unused_async)]
    async fn stop(&mut self, service: &str) -> pingora::Result<String> {
        todo!("stop {service} of {} is not implemented", Self::IDENT);
    }
}
