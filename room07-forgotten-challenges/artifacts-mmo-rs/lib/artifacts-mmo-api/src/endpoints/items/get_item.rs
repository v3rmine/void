use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        CraftSkillSchema, ItemSchema, ItemTypeSchema, MonsterSchema, PaginatedResponseSchema,
        ResourceSchema, ResponseSchema, SingleItemSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

#[derive(TypedBuilder)]
struct GetItemRequest {
    #[builder(setter(into))]
    code: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_item_items__code__get>
#[tracing::instrument(level = "trace")]
pub fn get_item(
    GetItemRequest { code }: GetItemRequest,
) -> Result<EncodedRequest<GetItemRequest>, crate::Error> {
    let code = Code::try_new(code)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/items/{code}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetItemRequest> {
    type Response = ResponseSchema<SingleItemSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_item_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetItemRequest::builder()
                .code(code)
                .build();
            assert!(super::get_item(request).is_ok());
        }
    }
}
