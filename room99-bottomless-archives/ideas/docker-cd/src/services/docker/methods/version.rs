// https://docs.docker.com/engine/api/v1.41/#operation/SystemVersion

use std::collections::HashMap;

use eyre::Result;
use serde::Deserialize;

use crate::services::docker::DockerConnection;

#[allow(dead_code)]
pub async fn version(docker_conn: &DockerConnection) -> Result<DockerVersion> {
    tracing::trace!("Getting docker version information");
    Ok(super::simple_get!(docker_conn, "/version", DockerVersion))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerVersion {
    pub platform: DockerVersionPlatform,
    pub components: Vec<DockerVersionPlatform>,
    pub version: String,
    pub api_version: String,
    #[serde(rename = "MinAPIVersion")]
    pub min_api_version: String,
    pub git_commit: String,
    pub go_version: String,
    pub os: String,
    pub arch: String,
    pub kernel_version: String,
    #[serde(default = "default_experimental")]
    pub experimental: bool,
    pub build_time: chrono::DateTime<chrono::Local>,
}

fn default_experimental() -> bool {
    false
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerVersionPlatform {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerVersionComponent {
    pub name: String,
    pub version: String,
    pub details: Option<HashMap<String, String>>,
}
