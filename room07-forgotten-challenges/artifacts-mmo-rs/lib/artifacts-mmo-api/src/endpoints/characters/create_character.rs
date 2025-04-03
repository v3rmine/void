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
pub struct CreateCharacterRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into))]
    skin: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/create_character_characters_create_post>
pub fn create_character(
    CreateCharacterRequest {
        bearer_token,
        name,
        skin,
    }: CreateCharacterRequest,
) -> Result<EncodedRequest<CreateCharacterRequest>, crate::Error> {
    let name = Name::try_new(name)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_static("/characters/create"),
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: serde_json::to_vec(&json!({
            "name": name,
            "skin": skin
        }))?,
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn create_character_should_work_with_valid_input(
            name in "[a-zA-Z0-9_-]+"
                .prop_filter(
                    "name must be at least 3 characters and at most 12",
                    |n| n.len() >= 3 && n.len() <= 12,
                ),
            skin in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::CreateCharacterRequest::builder()
                .bearer_token(BearerToken("a valid token".to_string()))
                .name(name)
                .skin(skin)
                .build();
            assert!(super::create_character(request).is_ok());
        }
    }
}
