// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Keys group results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectReleaseOrderBy {
    /// Order by the release date.
    #[default]
    ReleasedAt,
    /// Order by the creation date.
    CreatedAt,
}

impl ProjectReleaseOrderBy {
    /// The ordering as a query parameter.
    fn as_str(self) -> &'static str {
        match self {
            ProjectReleaseOrderBy::ReleasedAt => "released_at",
            ProjectReleaseOrderBy::CreatedAt => "created_at",
        }
    }
}

impl ParamValue<'static> for ProjectReleaseOrderBy {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query releases of a project.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectReleases<'a> {
    /// The project to query for releases.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Whether to include an HTML render of the description or not.
    #[builder(default)]
    include_html_description: Option<bool>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<ProjectReleaseOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> ProjectReleases<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectReleasesBuilder<'a> {
        ProjectReleasesBuilder::default()
    }
}

impl<'a> Endpoint for ProjectReleases<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/releases", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("include_html_description", self.include_html_description)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        params
    }
}

impl<'a> Pageable for ProjectReleases<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::common::SortOrder;
    use crate::api::projects::releases::{
        ProjectReleaseOrderBy, ProjectReleases, ProjectReleasesBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn order_by_default() {
        assert_eq!(
            ProjectReleaseOrderBy::default(),
            ProjectReleaseOrderBy::ReleasedAt,
        );
    }

    #[test]
    fn order_by_as_str() {
        let items = &[
            (ProjectReleaseOrderBy::ReleasedAt, "released_at"),
            (ProjectReleaseOrderBy::CreatedAt, "created_at"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = ProjectReleases::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, ProjectReleasesBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        ProjectReleases::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/project/releases")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectReleases::builder()
            .project("project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_html_description() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/project/releases")
            .add_query_params(&[("include_html_description", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectReleases::builder()
            .project("project")
            .include_html_description(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order_by() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/project/releases")
            .add_query_params(&[("order_by", "created_at")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectReleases::builder()
            .project("project")
            .order_by(ProjectReleaseOrderBy::CreatedAt)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_sort() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/project/releases")
            .add_query_params(&[("sort", "asc")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = ProjectReleases::builder()
            .project("project")
            .sort(SortOrder::Ascending)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
