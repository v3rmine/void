use std::{marker::PhantomData, str::FromStr};

use http::{header::ACCEPT, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{GEItemSchema, PaginatedResponseSchema, ResponseSchema},
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

#[derive(TypedBuilder)]
struct GetGEItemRequest {
    code: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_ge_item_ge__code__get>
#[tracing::instrument(level = "trace")]
pub fn get_ge_item(
    GetGEItemRequest { code }: GetGEItemRequest,
) -> Result<EncodedRequest<GetGEItemRequest>, crate::Error> {
    let code = Code::try_new(code)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/ge/{code}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetGEItemRequest> {
    type Response = ResponseSchema<GEItemSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_ge_item_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetGEItemRequest::builder()
                .code(code)
                .build();
            assert!(super::get_ge_item(request).is_ok());
        }
    }
}
