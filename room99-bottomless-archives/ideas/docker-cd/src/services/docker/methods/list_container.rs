// https://docs.docker.com/engine/api/v1.41/#operation/ContainerList

use std::collections::HashMap;

use eyre::Result;
use serde::Deserialize;

use crate::services::{
    docker::DockerConnection,
    uri::{bool_as_str, query_from_hash_map},
};

#[allow(dead_code)]
pub async fn list_container(docker_conn: &DockerConnection) -> Result<Vec<Container>> {
    tracing::trace!("Getting containers");
    Ok(super::simple_get!(
        docker_conn,
        "/containers/json",
        Vec<Container>
    ))
}

#[allow(dead_code)]
pub async fn list_container_with_params(
    docker_conn: &DockerConnection,
    params: &RequestListContainer,
) -> Result<Vec<Container>> {
    tracing::trace!("Getting containers");
    Ok(super::simple_get!(
        docker_conn,
        &["/containers/json", &query_from_hash_map(params)].concat(),
        Vec<Container>
    ))
}

#[derive(Debug, Clone, typed_builder::TypedBuilder)]
pub struct RequestListContainer {
    #[builder(default = true)]
    all: bool,
    #[builder(default, setter(strip_option))]
    limit: Option<i32>,
    #[builder(default = false)]
    size: bool,
    #[builder(default, setter(transform = |filters: &str| Some(filters.to_string())))]
    filters: Option<String>,
}

// TODO make a derive macro for this
impl<'any> From<&'any RequestListContainer> for HashMap<String, String> {
    fn from(src: &'any RequestListContainer) -> Self {
        let mut params: Self = HashMap::with_capacity(4);

        params.insert("all".to_string(), bool_as_str(src.all).to_string());
        if let Some(limit) = src.limit {
            params.insert("limit".to_string(), limit.to_string());
        }
        params.insert("size".to_string(), bool_as_str(src.size).to_string());
        if let Some(filters) = &src.filters {
            params.insert("filters".to_string(), filters.clone());
        }

        params
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub command: String,
    pub created: i64,
    pub ports: Vec<ContainerPort>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub labels: HashMap<String, String>,
    pub state: String,
    pub status: String,
    pub host_config: ContainerHostConfig,
    pub network_settings: ContainerNetworkSettings,
    pub mounts: Vec<ContainerMount>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerPort {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    #[serde(rename = "Type")]
    pub port_type: String, // tcp, udp, stcp
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerHostConfig {
    pub network_mode: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerMount {
    #[serde(rename = "Type")]
    pub mount_type: String, // bind, volume, tmpfs, npipe
    pub name: Option<String>,
    pub source: String,
    pub destination: String,
    pub driver: Option<String>,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub propagation: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerNetworkSettings {
    pub networks: HashMap<String, ContainerEndpointConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerEndpointConfig {
    pub links: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: i32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: i64,
    pub mac_address: String,
    pub driver_opts: Option<HashMap<String, String>>,
}
