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

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);

#[derive(TypedBuilder)]
pub struct GetAllCharactersRequest {
    #[builder(default = 1)]
    page: u32,
    #[builder(default = 50)]
    size: u32,
    #[builder(default)]
    sort: Option<CraftSkillSchema>,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_characters_characters__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_characters(
    GetAllCharactersRequest { page, size, sort }: GetAllCharactersRequest,
) -> Result<EncodedRequest<GetAllCharactersRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    let mut query = Vec::new();
    if let Some(sort) = sort {
        query.push(format!("sort={sort}"));
    }
    let path = format!("/characters/?page={page}&size={size}&{}", query.join("&"));

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&path)?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllCharactersRequest> {
    type Response = PaginatedResponseSchema<CharacterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::schemas::BearerToken;

    proptest! {
        #[test]
        fn get_all_characters_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            let request = super::GetAllCharactersRequest::builder()
                .page(page)
                .size(size)
                .build();
            assert!(super::get_all_characters(request).is_ok());
        }
    }
}
