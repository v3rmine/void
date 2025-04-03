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

#[derive(TypedBuilder)]
pub struct GetBankGoldsRequest {
    bearer_token: BearerToken,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_bank_golds_my_bank_gold_get>
pub fn get_bank_golds(
    GetBankGoldsRequest { bearer_token }: GetBankGoldsRequest,
) -> Result<EncodedRequest<GetBankGoldsRequest>, crate::Error> {
    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_static("/my/bank/gold"),
        headers: HeaderMap::from_iter([
            ACCEPT_JSON,
            (
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", bearer_token.0))?,
            ),
        ]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetBankGoldsRequest> {
    type Response = ResponseSchema<GoldSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    #[test]
    fn get_bank_golds_should_work_with_valid_input() {
        let request = super::GetBankGoldsRequest::builder()
            .bearer_token(BearerToken("a valid token".to_string()))
            .build();
        assert!(super::get_bank_golds(request).is_ok());
    }
}
