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
/// ResourceObject : An object describing the resources which can be advertised by a node and requested by a task.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs")]
    nano_cp_us: Option<i64>,
    #[serde(rename = "MemoryBytes")]
    memory_bytes: Option<i64>,
    #[serde(rename = "GenericResources")]
    generic_resources: Option<super::GenericResources>,
}

impl ResourceObject {
    /// An object describing the resources which can be advertised by a node and requested by a task.
    pub fn new() -> ResourceObject {
        ResourceObject {
            nano_cp_us: None,
            memory_bytes: None,
            generic_resources: None,
        }
    }

    pub fn set_nano_cp_us(&mut self, nano_cp_us: i64) {
        self.nano_cp_us = Some(nano_cp_us);
    }

    pub fn with_nano_cp_us(mut self, nano_cp_us: i64) -> ResourceObject {
        self.nano_cp_us = Some(nano_cp_us);
        self
    }

    pub fn nano_cp_us(&self) -> Option<&i64> {
        self.nano_cp_us.as_ref()
    }

    pub fn reset_nano_cp_us(&mut self) {
        self.nano_cp_us = None;
    }

    pub fn set_memory_bytes(&mut self, memory_bytes: i64) {
        self.memory_bytes = Some(memory_bytes);
    }

    pub fn with_memory_bytes(mut self, memory_bytes: i64) -> ResourceObject {
        self.memory_bytes = Some(memory_bytes);
        self
    }

    pub fn memory_bytes(&self) -> Option<&i64> {
        self.memory_bytes.as_ref()
    }

    pub fn reset_memory_bytes(&mut self) {
        self.memory_bytes = None;
    }

    pub fn set_generic_resources(&mut self, generic_resources: super::GenericResources) {
        self.generic_resources = Some(generic_resources);
    }

    pub fn with_generic_resources(
        mut self,
        generic_resources: super::GenericResources,
    ) -> ResourceObject {
        self.generic_resources = Some(generic_resources);
        self
    }

    pub fn generic_resources(&self) -> Option<&super::GenericResources> {
        self.generic_resources.as_ref()
    }

    pub fn reset_generic_resources(&mut self) {
        self.generic_resources = None;
    }
}
