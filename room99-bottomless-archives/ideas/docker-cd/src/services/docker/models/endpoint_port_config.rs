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
pub struct EndpointPortConfig {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Protocol")]
    protocol: Option<String>,
    /// The port inside the container.
    #[serde(rename = "TargetPort")]
    target_port: Option<i32>,
    /// The port on the swarm hosts.
    #[serde(rename = "PublishedPort")]
    published_port: Option<i32>,
    /// The mode in which port is published.    <p><br /></p>    - \"ingress\" makes the target port accessible on every node,     regardless of whether there is a task for the service running on     that node or not. - \"host\" bypasses the routing mesh and publish the port directly on     the swarm node where that service is running.
    #[serde(rename = "PublishMode")]
    publish_mode: Option<String>,
}

impl EndpointPortConfig {
    pub fn new() -> EndpointPortConfig {
        EndpointPortConfig {
            name: None,
            protocol: None,
            target_port: None,
            published_port: None,
            publish_mode: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> EndpointPortConfig {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = Some(protocol);
    }

    pub fn with_protocol(mut self, protocol: String) -> EndpointPortConfig {
        self.protocol = Some(protocol);
        self
    }

    pub fn protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    pub fn reset_protocol(&mut self) {
        self.protocol = None;
    }

    pub fn set_target_port(&mut self, target_port: i32) {
        self.target_port = Some(target_port);
    }

    pub fn with_target_port(mut self, target_port: i32) -> EndpointPortConfig {
        self.target_port = Some(target_port);
        self
    }

    pub fn target_port(&self) -> Option<&i32> {
        self.target_port.as_ref()
    }

    pub fn reset_target_port(&mut self) {
        self.target_port = None;
    }

    pub fn set_published_port(&mut self, published_port: i32) {
        self.published_port = Some(published_port);
    }

    pub fn with_published_port(mut self, published_port: i32) -> EndpointPortConfig {
        self.published_port = Some(published_port);
        self
    }

    pub fn published_port(&self) -> Option<&i32> {
        self.published_port.as_ref()
    }

    pub fn reset_published_port(&mut self) {
        self.published_port = None;
    }

    pub fn set_publish_mode(&mut self, publish_mode: String) {
        self.publish_mode = Some(publish_mode);
    }

    pub fn with_publish_mode(mut self, publish_mode: String) -> EndpointPortConfig {
        self.publish_mode = Some(publish_mode);
        self
    }

    pub fn publish_mode(&self) -> Option<&String> {
        self.publish_mode.as_ref()
    }

    pub fn reset_publish_mode(&mut self) {
        self.publish_mode = None;
    }
}
