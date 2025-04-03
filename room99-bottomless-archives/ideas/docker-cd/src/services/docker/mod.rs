use std::{fmt, path::PathBuf, sync::Arc};

use eyre::Result;
use hyper::{Client, Uri};

use self::{connector::DockerConnector, methods::RequestListContainer};

use super::env::var_not_empty;

mod connector;
mod methods;
mod models;

const UNIX_PATTERN: &str = "unix://";
const SOCKER_PATH: &str = "unix:///var/run/docker.sock";
const DEFAULT_URL: &str = "http://docker:2375";
const DOCKER_API_VERSION: &str = "v1.40";

// Inspired by https://crates.io/crates/bollard
// curl --unix-socket /var/run/docker.sock http:/v1.41/version
// curl http://docker:2375/v1.41/version
#[derive(typed_builder::TypedBuilder)]
pub struct DockerConnection {
    #[builder(default = DockerConnection::infer_docker_host())]
    docker_host: String,
    #[builder(default = Arc::new(DockerConnection::generate_client()), setter(skip))]
    client: Arc<Client<DockerConnector>>,
}

impl DockerConnection {
    #[tracing::instrument]
    pub fn infer_docker_host() -> String {
        let mut docker_host =
            var_not_empty("DOCKER_HOST").unwrap_or_else(|_| SOCKER_PATH.to_string());

        let is_unix_socket =
            docker_host.starts_with(UNIX_PATTERN) || docker_host.ends_with(".sock");
        let unix_socket_exists =
            PathBuf::from(docker_host.trim_start_matches(UNIX_PATTERN)).exists();

        if is_unix_socket && !unix_socket_exists {
            docker_host = DEFAULT_URL.to_string();
        }

        tracing::trace!("Inferred DOCKER_HOST as {}", docker_host);

        docker_host
    }

    pub fn generate_client() -> Client<DockerConnector> {
        Client::builder().build(DockerConnector::default())
    }

    #[tracing::instrument]
    pub fn format_uri(&self, path: &str) -> Result<Uri> {
        Ok(if self.docker_host.starts_with(UNIX_PATTERN) {
            tracing::trace!("Formatting an unix uri");
            hyperlocal::Uri::new(&self.docker_host.trim_start_matches(UNIX_PATTERN), path).into()
        } else {
            tracing::trace!("Formatting an http uri");
            format!("{}/{DOCKER_API_VERSION}{path}", self.docker_host).parse::<Uri>()?
        })
    }

    #[tracing::instrument]
    pub async fn version(&self) -> Result<methods::DockerVersion> {
        methods::version(self).await
    }

    #[tracing::instrument]
    pub async fn get_container_by_name<S>(&self, name: S) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        self.list_containers_with_params(
            &methods::RequestListContainer::builder()
                .all(true)
                .size(true)
                .limit(1)
                .filters(&format!("name={}", name.as_ref()))
                .build(),
        )
        .await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn list_containers(&self) -> Result<Vec<methods::Container>> {
        methods::list_container(self).await
    }

    #[tracing::instrument]
    pub async fn list_containers_with_params(
        &self,
        params: &RequestListContainer,
    ) -> Result<Vec<methods::Container>> {
        methods::list_container_with_params(self, params).await
    }

    #[tracing::instrument]
    pub fn rename_container<S>(&self, container_ident: S, new_name: S) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }

    #[tracing::instrument]
    pub fn start_container<S>(&self, container_ident: S) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }

    #[tracing::instrument]
    pub fn stop_container<S>(&self, container_ident: S) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }

    #[tracing::instrument]
    pub fn delete_container<S>(&self, container_ident: S) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }

    #[tracing::instrument]
    pub fn pull_image<S>(&self, image: S, docker_credentials: ()) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }

    #[tracing::instrument]
    pub fn create_container<S>(
        &self,
        image: S,
        name: S,
        ports: (),
        volumes: (),
        environment: (),
    ) -> Result<()>
    where
        S: AsRef<str>,
        S: fmt::Debug,
    {
        todo!()
    }
}

impl fmt::Debug for DockerConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DockerConnection")
            .field("docker_host", &self.docker_host)
            .finish()
    }
}

impl Default for DockerConnection {
    fn default() -> Self {
        Self::builder().build()
    }
}
