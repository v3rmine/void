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

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);
#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct ItemCode(String);

#[derive(TypedBuilder)]
pub struct GetBankItemsRequest {
    bearer_token: BearerToken,
    #[builder(default = 1)]
    page: u32,
    #[builder(default = 50)]
    size: u32,
    #[builder(default, setter(into))]
    item_code: Option<String>,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_bank_items_my_bank_items_get>
pub fn get_bank_items(
    GetBankItemsRequest {
        bearer_token,
        page,
        size,
        item_code,
    }: GetBankItemsRequest,
) -> Result<EncodedRequest<GetBankItemsRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    let mut query = Vec::new();
    if let Some(item_code) = item_code {
        query.push(format!("item_code={item_code}"));
    }
    let path = format!(
        "/my/bank/items/?page={page}&size={size}&{}",
        query.join("&")
    );

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&path)?,
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

impl<'de> ParseResponse<'de> for EncodedRequest<GetBankItemsRequest> {
    type Response = PaginatedResponseSchema<SimpleItemSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn get_bank_items_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            let request = super::GetBankItemsRequest::builder()
                .bearer_token(BearerToken("a valid token".to_string()))
                .page(page)
                .size(size)
                .build();
            assert!(super::get_bank_items(request).is_ok());
        }
    }
}
