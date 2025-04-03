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
/// EndpointSettings : Configuration for a network endpoint.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointSettings {
    #[serde(rename = "IPAMConfig")]
    ipam_config: Option<super::EndpointIpamConfig>,
    #[serde(rename = "Links")]
    links: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    aliases: Option<Vec<String>>,
    /// Unique ID of the network.
    #[serde(rename = "NetworkID")]
    network_id: Option<String>,
    /// Unique ID for the service endpoint in a Sandbox.
    #[serde(rename = "EndpointID")]
    endpoint_id: Option<String>,
    /// Gateway address for this network.
    #[serde(rename = "Gateway")]
    gateway: Option<String>,
    /// IPv4 address.
    #[serde(rename = "IPAddress")]
    ip_address: Option<String>,
    /// Mask length of the IPv4 address.
    #[serde(rename = "IPPrefixLen")]
    ip_prefix_len: Option<i32>,
    /// IPv6 gateway address.
    #[serde(rename = "IPv6Gateway")]
    i_pv6_gateway: Option<String>,
    /// Global IPv6 address.
    #[serde(rename = "GlobalIPv6Address")]
    global_i_pv6_address: Option<String>,
    /// Mask length of the global IPv6 address.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    global_i_pv6_prefix_len: Option<i64>,
    /// MAC address for the endpoint on this network.
    #[serde(rename = "MacAddress")]
    mac_address: Option<String>,
    /// DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    driver_opts: Option<::std::collections::HashMap<String, String>>,
}

impl EndpointSettings {
    /// Configuration for a network endpoint.
    pub fn new() -> EndpointSettings {
        EndpointSettings {
            ipam_config: None,
            links: None,
            aliases: None,
            network_id: None,
            endpoint_id: None,
            gateway: None,
            ip_address: None,
            ip_prefix_len: None,
            i_pv6_gateway: None,
            global_i_pv6_address: None,
            global_i_pv6_prefix_len: None,
            mac_address: None,
            driver_opts: None,
        }
    }

    pub fn set_ipam_config(&mut self, ipam_config: super::EndpointIpamConfig) {
        self.ipam_config = Some(ipam_config);
    }

    pub fn with_ipam_config(mut self, ipam_config: super::EndpointIpamConfig) -> EndpointSettings {
        self.ipam_config = Some(ipam_config);
        self
    }

    pub fn ipam_config(&self) -> Option<&super::EndpointIpamConfig> {
        self.ipam_config.as_ref()
    }

    pub fn reset_ipam_config(&mut self) {
        self.ipam_config = None;
    }

    pub fn set_links(&mut self, links: Vec<String>) {
        self.links = Some(links);
    }

    pub fn with_links(mut self, links: Vec<String>) -> EndpointSettings {
        self.links = Some(links);
        self
    }

    pub fn links(&self) -> Option<&Vec<String>> {
        self.links.as_ref()
    }

    pub fn reset_links(&mut self) {
        self.links = None;
    }

    pub fn set_aliases(&mut self, aliases: Vec<String>) {
        self.aliases = Some(aliases);
    }

    pub fn with_aliases(mut self, aliases: Vec<String>) -> EndpointSettings {
        self.aliases = Some(aliases);
        self
    }

    pub fn aliases(&self) -> Option<&Vec<String>> {
        self.aliases.as_ref()
    }

    pub fn reset_aliases(&mut self) {
        self.aliases = None;
    }

    pub fn set_network_id(&mut self, network_id: String) {
        self.network_id = Some(network_id);
    }

    pub fn with_network_id(mut self, network_id: String) -> EndpointSettings {
        self.network_id = Some(network_id);
        self
    }

    pub fn network_id(&self) -> Option<&String> {
        self.network_id.as_ref()
    }

    pub fn reset_network_id(&mut self) {
        self.network_id = None;
    }

    pub fn set_endpoint_id(&mut self, endpoint_id: String) {
        self.endpoint_id = Some(endpoint_id);
    }

    pub fn with_endpoint_id(mut self, endpoint_id: String) -> EndpointSettings {
        self.endpoint_id = Some(endpoint_id);
        self
    }

    pub fn endpoint_id(&self) -> Option<&String> {
        self.endpoint_id.as_ref()
    }

    pub fn reset_endpoint_id(&mut self) {
        self.endpoint_id = None;
    }

    pub fn set_gateway(&mut self, gateway: String) {
        self.gateway = Some(gateway);
    }

    pub fn with_gateway(mut self, gateway: String) -> EndpointSettings {
        self.gateway = Some(gateway);
        self
    }

    pub fn gateway(&self) -> Option<&String> {
        self.gateway.as_ref()
    }

    pub fn reset_gateway(&mut self) {
        self.gateway = None;
    }

    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = Some(ip_address);
    }

    pub fn with_ip_address(mut self, ip_address: String) -> EndpointSettings {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    pub fn reset_ip_address(&mut self) {
        self.ip_address = None;
    }

    pub fn set_ip_prefix_len(&mut self, ip_prefix_len: i32) {
        self.ip_prefix_len = Some(ip_prefix_len);
    }

    pub fn with_ip_prefix_len(mut self, ip_prefix_len: i32) -> EndpointSettings {
        self.ip_prefix_len = Some(ip_prefix_len);
        self
    }

    pub fn ip_prefix_len(&self) -> Option<&i32> {
        self.ip_prefix_len.as_ref()
    }

    pub fn reset_ip_prefix_len(&mut self) {
        self.ip_prefix_len = None;
    }

    pub fn set_i_pv6_gateway(&mut self, i_pv6_gateway: String) {
        self.i_pv6_gateway = Some(i_pv6_gateway);
    }

    pub fn with_i_pv6_gateway(mut self, i_pv6_gateway: String) -> EndpointSettings {
        self.i_pv6_gateway = Some(i_pv6_gateway);
        self
    }

    pub fn i_pv6_gateway(&self) -> Option<&String> {
        self.i_pv6_gateway.as_ref()
    }

    pub fn reset_i_pv6_gateway(&mut self) {
        self.i_pv6_gateway = None;
    }

    pub fn set_global_i_pv6_address(&mut self, global_i_pv6_address: String) {
        self.global_i_pv6_address = Some(global_i_pv6_address);
    }

    pub fn with_global_i_pv6_address(mut self, global_i_pv6_address: String) -> EndpointSettings {
        self.global_i_pv6_address = Some(global_i_pv6_address);
        self
    }

    pub fn global_i_pv6_address(&self) -> Option<&String> {
        self.global_i_pv6_address.as_ref()
    }

    pub fn reset_global_i_pv6_address(&mut self) {
        self.global_i_pv6_address = None;
    }

    pub fn set_global_i_pv6_prefix_len(&mut self, global_i_pv6_prefix_len: i64) {
        self.global_i_pv6_prefix_len = Some(global_i_pv6_prefix_len);
    }

    pub fn with_global_i_pv6_prefix_len(
        mut self,
        global_i_pv6_prefix_len: i64,
    ) -> EndpointSettings {
        self.global_i_pv6_prefix_len = Some(global_i_pv6_prefix_len);
        self
    }

    pub fn global_i_pv6_prefix_len(&self) -> Option<&i64> {
        self.global_i_pv6_prefix_len.as_ref()
    }

    pub fn reset_global_i_pv6_prefix_len(&mut self) {
        self.global_i_pv6_prefix_len = None;
    }

    pub fn set_mac_address(&mut self, mac_address: String) {
        self.mac_address = Some(mac_address);
    }

    pub fn with_mac_address(mut self, mac_address: String) -> EndpointSettings {
        self.mac_address = Some(mac_address);
        self
    }

    pub fn mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    pub fn reset_mac_address(&mut self) {
        self.mac_address = None;
    }

    pub fn set_driver_opts(&mut self, driver_opts: ::std::collections::HashMap<String, String>) {
        self.driver_opts = Some(driver_opts);
    }

    pub fn with_driver_opts(
        mut self,
        driver_opts: ::std::collections::HashMap<String, String>,
    ) -> EndpointSettings {
        self.driver_opts = Some(driver_opts);
        self
    }

    pub fn driver_opts(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.driver_opts.as_ref()
    }

    pub fn reset_driver_opts(&mut self) {
        self.driver_opts = None;
    }
}
