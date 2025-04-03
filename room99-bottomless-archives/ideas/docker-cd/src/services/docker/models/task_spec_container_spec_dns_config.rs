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
/// TaskSpecContainerSpecDnsConfig : Specification for DNS related configurations in resolver configuration file (`resolv.conf`).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers.
    #[serde(rename = "Nameservers")]
    nameservers: Option<Vec<String>>,
    /// A search list for host-name lookup.
    #[serde(rename = "Search")]
    search: Option<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.).
    #[serde(rename = "Options")]
    options: Option<Vec<String>>,
}

impl TaskSpecContainerSpecDnsConfig {
    /// Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
    pub fn new() -> TaskSpecContainerSpecDnsConfig {
        TaskSpecContainerSpecDnsConfig {
            nameservers: None,
            search: None,
            options: None,
        }
    }

    pub fn set_nameservers(&mut self, nameservers: Vec<String>) {
        self.nameservers = Some(nameservers);
    }

    pub fn with_nameservers(mut self, nameservers: Vec<String>) -> TaskSpecContainerSpecDnsConfig {
        self.nameservers = Some(nameservers);
        self
    }

    pub fn nameservers(&self) -> Option<&Vec<String>> {
        self.nameservers.as_ref()
    }

    pub fn reset_nameservers(&mut self) {
        self.nameservers = None;
    }

    pub fn set_search(&mut self, search: Vec<String>) {
        self.search = Some(search);
    }

    pub fn with_search(mut self, search: Vec<String>) -> TaskSpecContainerSpecDnsConfig {
        self.search = Some(search);
        self
    }

    pub fn search(&self) -> Option<&Vec<String>> {
        self.search.as_ref()
    }

    pub fn reset_search(&mut self) {
        self.search = None;
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = Some(options);
    }

    pub fn with_options(mut self, options: Vec<String>) -> TaskSpecContainerSpecDnsConfig {
        self.options = Some(options);
        self
    }

    pub fn options(&self) -> Option<&Vec<String>> {
        self.options.as_ref()
    }

    pub fn reset_options(&mut self) {
        self.options = None;
    }
}
