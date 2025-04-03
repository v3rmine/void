use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::ACTIONS_RATE_LIMIT,
    schemas::{BearerToken, CharacterMovementDataSchema, ResponseSchema},
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Name(String);

#[derive(TypedBuilder)]
pub struct ActionMoveRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    name: String,
    x: u32,
    y: u32,
}

pub fn action_move(
    ActionMoveRequest {
        bearer_token,
        name,
        x,
        y,
    }: ActionMoveRequest,
) -> Result<EncodedRequest<ActionMoveRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_str(&format!("/my/{name}/action/move"))?,
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            CONTENT_TYPE_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: serde_json::to_vec(&json!({
            "x": x,
            "y": y
        }))?,
        rate_limit: ACTIONS_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<ActionMoveRequest> {
    type Response = ResponseSchema<CharacterMovementDataSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;
    proptest! {
        #[test]
        fn action_move_should_not_panic_with_valid_input(
            name in "[a-zA-Z0-9_-]+",
            x in 0u32..=u32::MAX,
            y in 0u32..=u32::MAX
        ) {
            let request = super::ActionMoveRequest::builder()
                .bearer_token(BearerToken("valid token".to_string()))
                .name(name)
                .x(x)
                .y(y)
                .build();
            assert!(super::action_move(request).is_ok());
        }
    }
}
