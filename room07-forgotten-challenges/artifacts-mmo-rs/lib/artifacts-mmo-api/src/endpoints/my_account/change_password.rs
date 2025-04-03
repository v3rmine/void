use std::{marker::PhantomData, str::FromStr};

use http::{header::AUTHORIZATION, uri::PathAndQuery, HeaderMap, HeaderValue, Method};
use nutype::nutype;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        BearerToken, GoldSchema, MessageSchema, PaginatedResponseSchema, ResponseSchema,
        SimpleItemSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(not_empty, regex = "^[^\\s]+$", len_char_min = 5, len_char_max = 50))]
struct Password(String);

#[derive(TypedBuilder)]
pub struct ChangePasswordRequest {
    bearer_token: BearerToken,
    #[builder(setter(into))]
    password: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/change_password_my_change_password_post>
pub fn change_password(
    ChangePasswordRequest {
        bearer_token,
        password,
    }: ChangePasswordRequest,
) -> Result<EncodedRequest<ChangePasswordRequest>, crate::Error> {
    let password = Password::try_new(password)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::POST,
        path: PathAndQuery::from_static("/my/change_password"),
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: serde_json::to_vec(&json!({
            "password": password
        }))?,
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<ChangePasswordRequest> {
    type Response = MessageSchema;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn change_password_should_work_with_valid_input(
            password in "[^\\s]+"
                .prop_filter(
                    "password must be at least 5 and at most 50 characters",
                    |p| p.chars().count() >= 5 && p.chars().count() <= 50
                )
        ) {
            let request = super::ChangePasswordRequest::builder()
                .bearer_token(BearerToken("a valid token".to_string()))
                .password(password)
                .build();
            assert!(super::change_password(request).is_ok());
        }
    }
}
