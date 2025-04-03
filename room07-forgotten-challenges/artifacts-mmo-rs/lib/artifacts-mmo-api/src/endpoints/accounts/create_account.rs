use std::marker::PhantomData;

use http::{
    header::{ACCEPT, CONTENT_TYPE},
    uri::PathAndQuery,
    HeaderMap, HeaderValue, Method,
};
use nutype::nutype;
use serde::Deserialize;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
    helpers::{ACCEPT_JSON, CONTENT_TYPE_JSON},
    rate_limits::ACCOUNT_CREATION_RATE_LIMIT,
    schemas::MessageSchema,
    EncodedRequest, ParseResponse,
};

#[nutype(validate(
    not_empty,
    regex = "^[a-zA-Z0-9_-]+$",
    len_char_min = 6,
    len_char_max = 32
))]
struct Username(String);
#[nutype(validate(not_empty, regex = "^[^\\s]+$", len_char_min = 5, len_char_max = 50))]
struct Password(String);
#[nutype(validate(not_empty, regex = "^\\w+@\\w+\\.\\w+$"))]
struct Email(String);

#[derive(TypedBuilder)]
pub struct CreateAccountRequest {
    username: String,
    password: String,
    email: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/create_account_accounts_create_post>
#[tracing::instrument(level = "trace", skip_all)]
pub fn create_account(
    CreateAccountRequest {
        username,
        password,
        email,
    }: CreateAccountRequest,
) -> Result<EncodedRequest<CreateAccountRequest>, crate::Error> {
    let username = Username::try_new(username)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let password = Password::try_new(password)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let email = Email::try_new(email)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        path: PathAndQuery::from_static("/accounts/create"),
        method: Method::POST,
        headers: HeaderMap::from_iter([ACCEPT_JSON, CONTENT_TYPE_JSON]),
        content: serde_json::to_vec(&json!({
            "username": username,
            "password": password,
            "email": email
        }))?,
        rate_limit: ACCOUNT_CREATION_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<CreateAccountRequest> {
    type Response = MessageSchema;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn create_account_should_work_with_valid_input(
            username in "[a-zA-Z0-9_-]+"
                .prop_filter("at least 6 chars, at most 32", |v| v.len() >= 6 && v.len() <= 32),
            password in "[^\\s]+"
                // We use chars().count() because it can contains unicode characters
                .prop_filter("at least 5 chars and at most 50", |v| v.chars().count() >= 5 && v.chars().count() <= 50),
            email in "\\w+@\\w+\\.\\w+"
        ) {
            let request = super::CreateAccountRequest::builder()
                .username(username)
                .password(password)
                .email(email)
                .build();
            assert!(super::create_account(request).is_ok());
        }
    }
}
