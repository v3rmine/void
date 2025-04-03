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
/// HistoryResponseItem : individual image layer information in response to ImageHistory operation

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponseItem {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Created")]
    created: i64,
    #[serde(rename = "CreatedBy")]
    created_by: String,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "Comment")]
    comment: String,
}

impl HistoryResponseItem {
    /// individual image layer information in response to ImageHistory operation
    pub fn new(
        id: String,
        created: i64,
        created_by: String,
        tags: Vec<String>,
        size: i64,
        comment: String,
    ) -> HistoryResponseItem {
        HistoryResponseItem {
            id: id,
            created: created,
            created_by: created_by,
            tags: tags,
            size: size,
            comment: comment,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn with_id(mut self, id: String) -> HistoryResponseItem {
        self.id = id;
        self
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_created(&mut self, created: i64) {
        self.created = created;
    }

    pub fn with_created(mut self, created: i64) -> HistoryResponseItem {
        self.created = created;
        self
    }

    pub fn created(&self) -> &i64 {
        &self.created
    }

    pub fn set_created_by(&mut self, created_by: String) {
        self.created_by = created_by;
    }

    pub fn with_created_by(mut self, created_by: String) -> HistoryResponseItem {
        self.created_by = created_by;
        self
    }

    pub fn created_by(&self) -> &String {
        &self.created_by
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> HistoryResponseItem {
        self.tags = tags;
        self
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn set_size(&mut self, size: i64) {
        self.size = size;
    }

    pub fn with_size(mut self, size: i64) -> HistoryResponseItem {
        self.size = size;
        self
    }

    pub fn size(&self) -> &i64 {
        &self.size
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }

    pub fn with_comment(mut self, comment: String) -> HistoryResponseItem {
        self.comment = comment;
        self
    }

    pub fn comment(&self) -> &String {
        &self.comment
    }
}
