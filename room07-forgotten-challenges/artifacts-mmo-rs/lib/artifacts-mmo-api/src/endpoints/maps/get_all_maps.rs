use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        MapContentTypeSchema, MapSchema, MonsterSchema, PaginatedResponseSchema, ResourceSchema,
        ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(greater_or_equal = 1))]
struct Page(u32);
#[nutype(validate(greater_or_equal = 1, less_or_equal = 100))]
struct Size(u32);
#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct ContentCode(String);

#[derive(TypedBuilder)]
struct GetAllMapsRequest {
    #[builder(default = 1)]
    page: u32,
    #[builder(default = 50)]
    size: u32,
    #[builder(default, setter(into))]
    content_code: Option<String>,
    #[builder(default)]
    content_type: Option<MapContentTypeSchema>,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_all_maps_maps__get>
#[tracing::instrument(level = "trace")]
pub fn get_all_maps(
    GetAllMapsRequest {
        page,
        size,
        content_code,
        content_type,
    }: GetAllMapsRequest,
) -> Result<EncodedRequest<GetAllMapsRequest>, crate::Error> {
    let page = Page::try_new(page)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();
    let size = Size::try_new(size)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    let mut query = Vec::new();
    if let Some(content_code) = content_code {
        let content_code = ContentCode::try_new(content_code)
            .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
            .into_inner();
        query.push(format!("content_code={content_code}"));
    }
    if let Some(content_type) = content_type {
        query.push(format!("content_type={content_type}"));
    }
    let path = format!("/maps/?page={page}&size={size}&{}", query.join("&"));

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&path)?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetAllMapsRequest> {
    type Response = PaginatedResponseSchema<MonsterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_all_monsters_should_work_with_valid_input(
            page in 1u32..=u32::MAX,
            size in 1u32..=50,
        ) {
            let request = super::GetAllMapsRequest::builder()
                .page(page)
                .size(size)
                .build();
            assert!(super::get_all_maps(request).is_ok());
        }
    }
}
