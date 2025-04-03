use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::ACTIONS_RATE_LIMIT,
    schemas::{
        BearerToken, CharacterMovementDataSchema, EquipRequestSchema, ResponseSchema,
        SlotTypeSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Name(String);
#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

#[derive(TypedBuilder)]
pub struct ActionEquipItemRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into))]
    code: String,
    slot: SlotTypeSchema,
}

pub fn action_equip_item(
    ActionEquipItemRequest {
        bearer_token,
        name,
        code,
        slot,
    }: ActionEquipItemRequest,
) -> Result<EncodedRequest<ActionEquipItemRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_str(&format!("/my/{name}/action/equip"))?,
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: serde_json::to_vec(&json!({
            "code": code,
            "slot": slot
        }))?,
        rate_limit: ACTIONS_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<ActionEquipItemRequest> {
    type Response = ResponseSchema<EquipRequestSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::{BearerToken, SlotTypeSchema};
    proptest! {
        #[test]
        fn action_equip_should_not_panic_with_valid_input(
            name in "[a-zA-Z0-9_-]+",
            code in "[a-zA-Z0-9_-]+",
        ) {
            let request = super::ActionEquipItemRequest::builder()
                .bearer_token(BearerToken("valid token".to_string()))
                .name(name)
                .code(code)
                .slot(SlotTypeSchema::Weapon)
                .build();
            assert!(super::action_equip_item(request).is_ok());
        }
    }
}
