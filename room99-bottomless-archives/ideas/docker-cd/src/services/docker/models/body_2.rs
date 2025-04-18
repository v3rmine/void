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
pub struct Body2 {
    /// Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
    #[serde(rename = "ListenAddr")]
    listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr")]
    advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,    `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same addres as `AdvertiseAddr` is used.    The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr")]
    data_path_addr: Option<String>,
    /// Addresses of manager nodes already participating in the swarm.
    #[serde(rename = "RemoteAddrs")]
    remote_addrs: Option<Vec<String>>,
    /// Secret token for joining this swarm.
    #[serde(rename = "JoinToken")]
    join_token: Option<String>,
}

impl Body2 {
    pub fn new() -> Body2 {
        Body2 {
            listen_addr: None,
            advertise_addr: None,
            data_path_addr: None,
            remote_addrs: None,
            join_token: None,
        }
    }

    pub fn set_listen_addr(&mut self, listen_addr: String) {
        self.listen_addr = Some(listen_addr);
    }

    pub fn with_listen_addr(mut self, listen_addr: String) -> Body2 {
        self.listen_addr = Some(listen_addr);
        self
    }

    pub fn listen_addr(&self) -> Option<&String> {
        self.listen_addr.as_ref()
    }

    pub fn reset_listen_addr(&mut self) {
        self.listen_addr = None;
    }

    pub fn set_advertise_addr(&mut self, advertise_addr: String) {
        self.advertise_addr = Some(advertise_addr);
    }

    pub fn with_advertise_addr(mut self, advertise_addr: String) -> Body2 {
        self.advertise_addr = Some(advertise_addr);
        self
    }

    pub fn advertise_addr(&self) -> Option<&String> {
        self.advertise_addr.as_ref()
    }

    pub fn reset_advertise_addr(&mut self) {
        self.advertise_addr = None;
    }

    pub fn set_data_path_addr(&mut self, data_path_addr: String) {
        self.data_path_addr = Some(data_path_addr);
    }

    pub fn with_data_path_addr(mut self, data_path_addr: String) -> Body2 {
        self.data_path_addr = Some(data_path_addr);
        self
    }

    pub fn data_path_addr(&self) -> Option<&String> {
        self.data_path_addr.as_ref()
    }

    pub fn reset_data_path_addr(&mut self) {
        self.data_path_addr = None;
    }

    pub fn set_remote_addrs(&mut self, remote_addrs: Vec<String>) {
        self.remote_addrs = Some(remote_addrs);
    }

    pub fn with_remote_addrs(mut self, remote_addrs: Vec<String>) -> Body2 {
        self.remote_addrs = Some(remote_addrs);
        self
    }

    pub fn remote_addrs(&self) -> Option<&Vec<String>> {
        self.remote_addrs.as_ref()
    }

    pub fn reset_remote_addrs(&mut self) {
        self.remote_addrs = None;
    }

    pub fn set_join_token(&mut self, join_token: String) {
        self.join_token = Some(join_token);
    }

    pub fn with_join_token(mut self, join_token: String) -> Body2 {
        self.join_token = Some(join_token);
        self
    }

    pub fn join_token(&self) -> Option<&String> {
        self.join_token.as_ref()
    }

    pub fn reset_join_token(&mut self) {
        self.join_token = None;
    }
}
