use std::marker::PhantomData;

use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::NO_RATE_LIMIT,
    schemas::{ResponseSchema, StatusSchema},
    EncodedRequest, ParseResponse,
};

struct GetStatusRequest;
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_status__get>
#[tracing::instrument(level = "trace")]
pub fn get_status() -> EncodedRequest<GetStatusRequest> {
    EncodedRequest {
        path: PathAndQuery::from_static("/"),
        method: Method::GET,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: NO_RATE_LIMIT,
        marker: PhantomData,
    }
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetStatusRequest> {
    type Response = ResponseSchema<StatusSchema>;
}
