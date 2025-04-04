/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.    Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.    # Errors    The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:    ``` {     \"message\": \"page not found\" } ```    # Versioning    The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.    If you omit the version-prefix, the current version of the API (v1.40) is used. For example, calling `/info` is the same as calling `/v1.40/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.    Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.    The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.     # Authentication    Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:    ``` {     \"username\": \"string\",     \"password\": \"string\",     \"email\": \"string\",     \"serveraddress\": \"string\" } ```    The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.    If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:    ``` {     \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.40
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */
#![allow(
    dead_code,
    non_snake_case,
    clippy::redundant_field_names,
    clippy::too_many_arguments
)]

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerSummaryInner {
    /// The ID of this container
    #[serde(rename = "Id")]
    id: Option<String>,
    /// The names that this container has been given
    #[serde(rename = "Names")]
    names: Option<Vec<String>>,
    /// The name of the image used when creating this container
    #[serde(rename = "Image")]
    image: Option<String>,
    /// The ID of the image that this container was created from
    #[serde(rename = "ImageID")]
    image_id: Option<String>,
    /// Command to run when starting the container
    #[serde(rename = "Command")]
    command: Option<String>,
    /// When the container was created
    #[serde(rename = "Created")]
    created: Option<i64>,
    /// The ports exposed by this container
    #[serde(rename = "Ports")]
    ports: Option<Vec<super::Port>>,
    /// The size of files that have been created or changed by this container
    #[serde(rename = "SizeRw")]
    size_rw: Option<i64>,
    /// The total size of all the files in this container
    #[serde(rename = "SizeRootFs")]
    size_root_fs: Option<i64>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    labels: Option<::std::collections::HashMap<String, String>>,
    /// The state of this container (e.g. `Exited`)
    #[serde(rename = "State")]
    state: Option<String>,
    /// Additional human-readable status of this container (e.g. `Exit 0`)
    #[serde(rename = "Status")]
    status: Option<String>,
    #[serde(rename = "HostConfig")]
    host_config: Option<super::ContainerSummaryInnerHostConfig>,
    #[serde(rename = "NetworkSettings")]
    network_settings: Option<super::ContainerSummaryInnerNetworkSettings>,
    #[serde(rename = "Mounts")]
    mounts: Option<Vec<super::Mount>>,
}

impl ContainerSummaryInner {
    pub fn new() -> ContainerSummaryInner {
        ContainerSummaryInner {
            id: None,
            names: None,
            image: None,
            image_id: None,
            command: None,
            created: None,
            ports: None,
            size_rw: None,
            size_root_fs: None,
            labels: None,
            state: None,
            status: None,
            host_config: None,
            network_settings: None,
            mounts: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn with_id(mut self, id: String) -> ContainerSummaryInner {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn reset_id(&mut self) {
        self.id = None;
    }

    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = Some(names);
    }

    pub fn with_names(mut self, names: Vec<String>) -> ContainerSummaryInner {
        self.names = Some(names);
        self
    }

    pub fn names(&self) -> Option<&Vec<String>> {
        self.names.as_ref()
    }

    pub fn reset_names(&mut self) {
        self.names = None;
    }

    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }

    pub fn with_image(mut self, image: String) -> ContainerSummaryInner {
        self.image = Some(image);
        self
    }

    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn reset_image(&mut self) {
        self.image = None;
    }

    pub fn set_image_id(&mut self, image_id: String) {
        self.image_id = Some(image_id);
    }

    pub fn with_image_id(mut self, image_id: String) -> ContainerSummaryInner {
        self.image_id = Some(image_id);
        self
    }

    pub fn image_id(&self) -> Option<&String> {
        self.image_id.as_ref()
    }

    pub fn reset_image_id(&mut self) {
        self.image_id = None;
    }

    pub fn set_command(&mut self, command: String) {
        self.command = Some(command);
    }

    pub fn with_command(mut self, command: String) -> ContainerSummaryInner {
        self.command = Some(command);
        self
    }

    pub fn command(&self) -> Option<&String> {
        self.command.as_ref()
    }

    pub fn reset_command(&mut self) {
        self.command = None;
    }

    pub fn set_created(&mut self, created: i64) {
        self.created = Some(created);
    }

    pub fn with_created(mut self, created: i64) -> ContainerSummaryInner {
        self.created = Some(created);
        self
    }

    pub fn created(&self) -> Option<&i64> {
        self.created.as_ref()
    }

    pub fn reset_created(&mut self) {
        self.created = None;
    }

    pub fn set_ports(&mut self, ports: Vec<super::Port>) {
        self.ports = Some(ports);
    }

    pub fn with_ports(mut self, ports: Vec<super::Port>) -> ContainerSummaryInner {
        self.ports = Some(ports);
        self
    }

    pub fn ports(&self) -> Option<&Vec<super::Port>> {
        self.ports.as_ref()
    }

    pub fn reset_ports(&mut self) {
        self.ports = None;
    }

    pub fn set_size_rw(&mut self, size_rw: i64) {
        self.size_rw = Some(size_rw);
    }

    pub fn with_size_rw(mut self, size_rw: i64) -> ContainerSummaryInner {
        self.size_rw = Some(size_rw);
        self
    }

    pub fn size_rw(&self) -> Option<&i64> {
        self.size_rw.as_ref()
    }

    pub fn reset_size_rw(&mut self) {
        self.size_rw = None;
    }

    pub fn set_size_root_fs(&mut self, size_root_fs: i64) {
        self.size_root_fs = Some(size_root_fs);
    }

    pub fn with_size_root_fs(mut self, size_root_fs: i64) -> ContainerSummaryInner {
        self.size_root_fs = Some(size_root_fs);
        self
    }

    pub fn size_root_fs(&self) -> Option<&i64> {
        self.size_root_fs.as_ref()
    }

    pub fn reset_size_root_fs(&mut self) {
        self.size_root_fs = None;
    }

    pub fn set_labels(&mut self, labels: ::std::collections::HashMap<String, String>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: ::std::collections::HashMap<String, String>,
    ) -> ContainerSummaryInner {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_state(&mut self, state: String) {
        self.state = Some(state);
    }

    pub fn with_state(mut self, state: String) -> ContainerSummaryInner {
        self.state = Some(state);
        self
    }

    pub fn state(&self) -> Option<&String> {
        self.state.as_ref()
    }

    pub fn reset_state(&mut self) {
        self.state = None;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> ContainerSummaryInner {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_host_config(&mut self, host_config: super::ContainerSummaryInnerHostConfig) {
        self.host_config = Some(host_config);
    }

    pub fn with_host_config(
        mut self,
        host_config: super::ContainerSummaryInnerHostConfig,
    ) -> ContainerSummaryInner {
        self.host_config = Some(host_config);
        self
    }

    pub fn host_config(&self) -> Option<&super::ContainerSummaryInnerHostConfig> {
        self.host_config.as_ref()
    }

    pub fn reset_host_config(&mut self) {
        self.host_config = None;
    }

    pub fn set_network_settings(
        &mut self,
        network_settings: super::ContainerSummaryInnerNetworkSettings,
    ) {
        self.network_settings = Some(network_settings);
    }

    pub fn with_network_settings(
        mut self,
        network_settings: super::ContainerSummaryInnerNetworkSettings,
    ) -> ContainerSummaryInner {
        self.network_settings = Some(network_settings);
        self
    }

    pub fn network_settings(&self) -> Option<&super::ContainerSummaryInnerNetworkSettings> {
        self.network_settings.as_ref()
    }

    pub fn reset_network_settings(&mut self) {
        self.network_settings = None;
    }

    pub fn set_mounts(&mut self, mounts: Vec<super::Mount>) {
        self.mounts = Some(mounts);
    }

    pub fn with_mounts(mut self, mounts: Vec<super::Mount>) -> ContainerSummaryInner {
        self.mounts = Some(mounts);
        self
    }

    pub fn mounts(&self) -> Option<&Vec<super::Mount>> {
        self.mounts.as_ref()
    }

    pub fn reset_mounts(&mut self) {
        self.mounts = None;
    }
}
