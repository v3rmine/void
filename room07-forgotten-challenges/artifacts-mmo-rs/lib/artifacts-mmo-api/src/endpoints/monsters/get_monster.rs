use std::{marker::PhantomData, str::FromStr};

use http::{uri::PathAndQuery, HeaderMap, Method};
use nutype::nutype;
use typed_builder::TypedBuilder;

use crate::{
    helpers::ACCEPT_JSON,
    rate_limits::DATA_RATE_LIMIT,
    schemas::{
        MonsterSchema, PaginatedResponseSchema, ResourceSchema, ResponseSchema, SkillSchema,
    },
    EncodedRequest, ParseResponse,
};

#[nutype(validate(regex = "^[a-zA-Z0-9_-]+$"))]
struct Code(String);

#[derive(TypedBuilder)]
struct GetMonsterRequest {
    #[builder(setter(into))]
    code: String,
}
/// SOURCE: <https://api.artifactsmmo.com/docs/#/operations/get_monster_monsters__code__get>
#[tracing::instrument(level = "trace")]
pub fn get_monster(
    GetMonsterRequest { code }: GetMonsterRequest,
) -> Result<EncodedRequest<GetMonsterRequest>, crate::Error> {
    let code = Code::try_new(code)
        .map_err(|e| crate::Error::InvalidInput(e.to_string()))?
        .into_inner();

    Ok(EncodedRequest {
        method: Method::GET,
        path: PathAndQuery::from_str(&format!("/monsters/{code}"))?,
        headers: HeaderMap::from_iter([ACCEPT_JSON]),
        content: Vec::new(),
        rate_limit: DATA_RATE_LIMIT,
        marker: PhantomData,
    })
}

impl<'de> ParseResponse<'de> for EncodedRequest<GetMonsterRequest> {
    type Response = ResponseSchema<MonsterSchema>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn get_monster_should_work_with_valid_input(
            code in "[a-zA-Z0-9_-]+"
        ) {
            let request = super::GetMonsterRequest::builder()
                .code(code)
                .build();
            assert!(super::get_monster(request).is_ok());
        }
    }
}
