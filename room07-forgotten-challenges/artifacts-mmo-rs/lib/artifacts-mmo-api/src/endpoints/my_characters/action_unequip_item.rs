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

#[derive(TypedBuilder)]
pub struct ActionUnequipItemRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
    slot: SlotTypeSchema,
}

pub fn action_unequip_item(
    ActionUnequipItemRequest {
        bearer_token,
        name,
        slot,
    }: ActionUnequipItemRequest,
) -> Result<EncodedRequest<ActionUnequipItemRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_str(&format!("/my/{name}/action/unequip"))?,
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: serde_json::to_vec(&json!({
            "slot": slot
        }))?,
        rate_limit: ACTIONS_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<ActionUnequipItemRequest> {
    type Response = ResponseSchema<EquipRequestSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::{BearerToken, SlotTypeSchema};
    proptest! {
        #[test]
        fn action_unequip_should_not_panic_with_valid_input(
            name in "[a-zA-Z0-9_-]+",
        ) {
            let request = super::ActionUnequipItemRequest::builder()
                .bearer_token(BearerToken("valid token".to_string()))
                .name(name)
                .slot(SlotTypeSchema::Weapon)
                .build();
            assert!(super::action_unequip_item(request).is_ok());
        }
    }
}
