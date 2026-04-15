// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::SortOrder;
use crate::api::endpoint_prelude::*;
use crate::api::{common::NameOrId, ParamValue};

/// Orders commits may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagsOrderBy {
    /// Commits are returned in reverse chronological order.
    #[default]
    Name,
    /// Commits are returned in topological order.
    Updated,
}

impl TagsOrderBy {
    fn as_str(self) -> &'static str {
        match self {
            TagsOrderBy::Name => "name",
            TagsOrderBy::Updated => "updated",
        }
    }
}

impl ParamValue<'static> for TagsOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for a specific branch in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Tags<'a> {
    /// The project to get a branch from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// What field to order returned tags by
    #[builder(setter(into), default)]
    order_by: Option<TagsOrderBy>,

    /// Which order to sort the results
    #[builder(setter(into), default)]
    sort: Option<SortOrder>,

    /// Filter tags by a search query.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
}

impl<'a> Tags<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TagsBuilder<'a> {
        TagsBuilder::default()
    }
}

impl<'a> Endpoint for Tags<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/tags", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("search", self.search.as_ref())
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for Tags<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::SortOrder;
    use crate::api::projects::repository::tags::tags::{Tags, TagsBuilderError, TagsOrderBy};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn tags_order_by_default() {
        assert_eq!(TagsOrderBy::default(), TagsOrderBy::Name);
    }

    #[test]
    fn tags_order_as_str() {
        let items = &[
            (TagsOrderBy::Name, "name"),
            (TagsOrderBy::Updated, "updated"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_necessary() {
        let err = Tags::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, TagsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Tags::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/tags")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tags::builder().project("simple/project").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/tags")
            .add_query_params(&[("order_by", "updated")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tags::builder()
            .project("simple/project")
            .order_by(TagsOrderBy::Updated)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/tags")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tags::builder()
            .project("simple/project")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/tags")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Tags::builder()
            .project("simple/project")
            .search("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
