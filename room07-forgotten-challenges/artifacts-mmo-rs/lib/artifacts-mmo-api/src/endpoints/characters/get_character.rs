use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        BearerToken, CharacterSchema, CraftSkillSchema, MonsterSchema, PaginatedResponseSchema,
        ResourceSchema, ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Name(String);

#[derive(TypedBuilder)]
pub struct GetCharacterRequest {
    #[builder(setter(into))]
    name: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_character_characters__name__get>
#[tracing::instrument(level = "trace")]
pub fn get_character(
    GetCharacterRequest { name }: GetCharacterRequest,
) -> Result<EncodedRequest<GetCharacterRequest>, crate::Error> {
    let name = Name::try_new(name)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/characters/{name}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetCharacterRequest> {
    type Response = ResponseSchema<CharacterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn get_character_should_work_with_valid_input(
            name in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetCharacterRequest::builder()
                .name(name)
                .build();
            assert!(super::get_character(request).is_ok());
        }
    }
}
