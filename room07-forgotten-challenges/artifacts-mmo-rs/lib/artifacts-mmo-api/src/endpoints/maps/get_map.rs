use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        MapContentTypeSchema, MapSchema, MonsterSchema, PaginatedResponseSchema, ResourceSchema,
        ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[derive(TypedBuilder)]
struct GetMapRequest {
    x: u32,
    y: u32,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_map_maps__x___y__get>
#[tracing::instrument(level = "trace")]
pub fn get_map(
    GetMapRequest { x, y }: GetMapRequest,
) -> Result<EncodedRequest<GetMapRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/monsters/{x}/{y}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetMapRequest> {
    type Response = ResponseSchema<MapSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_map_should_work_with_valid_input(
            x in 1u32..=u32::MAX,
            y in 1u32..=u32::MAX,
        ) {
            let request = super::GetMapRequest::builder()
                .x(x)
                .y(y)
                .build();
            assert!(super::get_map(request).is_ok());
        }
    }
}
