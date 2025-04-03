use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::ACTIONS_RATE_LIMIT,
    schemas::{
        BearerToken, CharacterFightDataSchema, CharacterMovementDataSchema, EquipRequestSchema,
        ResponseSchema, SlotTypeSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Name(String);

#[derive(TypedBuilder)]
pub struct ActionFightRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
}

pub fn action_fight(
    ActionFightRequest { bearer_token, name }: ActionFightRequest,
) -> Result<EncodedRequest<ActionFightRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_str(&format!("/my/{name}/action/fight"))?,
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: Vec::new(),
        rate_limit: ACTIONS_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<ActionFightRequest> {
    type Response = ResponseSchema<CharacterFightDataSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::{BearerToken, SlotTypeSchema};
    proptest! {
        #[test]
        fn action_fight_should_not_panic_with_valid_input(
            name in "[a-zA-Z0-9_-]+",
        ) {
            let request = super::ActionFightRequest::builder()
                .bearer_token(BearerToken("valid token".to_string()))
                .name(name)
                .build();
            assert!(super::action_fight(request).is_ok());
        }
    }
}
