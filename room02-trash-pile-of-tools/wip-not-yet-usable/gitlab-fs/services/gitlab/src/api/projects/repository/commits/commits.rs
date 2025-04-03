// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Orders commits may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommitsOrder {
    /// Commits are returned in reverse chronological order.
    #[default]
    Default,
    /// Commits are returned in topological order.
    Topo,
}

impl CommitsOrder {
    fn as_str(self) -> &'static str {
        match self {
            CommitsOrder::Default => "default",
            CommitsOrder::Topo => "topo",
        }
    }
}

impl ParamValue<'static> for CommitsOrder {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for commits in a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Commits<'a> {
    /// The project to get commits from.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The ref to get commits from.
    ///
    /// If not given, the default branch will be used.
    #[builder(default, setter(into))]
    ref_name: Option<Cow<'a, str>>,
    /// Only return commits after a given date.
    #[builder(default)]
    since: Option<DateTime<Utc>>,
    /// Only return commits before a given date.
    #[builder(default)]
    until: Option<DateTime<Utc>>,
    /// Only return commits which affect a given path.
    #[builder(default, setter(into))]
    path: Option<Cow<'a, str>>,
    /// If true, return every commit from the repository.
    #[builder(default)]
    all: Option<bool>,
    /// Include commit stats in each commit object.
    #[builder(default)]
    with_stats: Option<bool>,
    /// If true, only consider commits in the first parent history.
    #[builder(default)]
    first_parent: Option<bool>,
    /// If true, only consider commits in the first parent history.
    #[builder(default)]
    order: Option<CommitsOrder>,
    /// If true, parse and include trailers from commit messages.
    #[builder(default)]
    trailers: Option<bool>,
}

impl<'a> Commits<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CommitsBuilder<'a> {
        CommitsBuilder::default()
    }
}

impl<'a> Endpoint for Commits<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/repository/commits", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params
            .push_opt("ref_name", self.ref_name.as_ref())
            .push_opt("since", self.since)
            .push_opt("until", self.until)
            .push_opt("path", self.path.as_ref())
            .push_opt("all", self.all)
            .push_opt("with_stats", self.with_stats)
            .push_opt("first_parent", self.first_parent)
            .push_opt("order", self.order)
            .push_opt("trailers", self.trailers);

        params
    }
}

impl<'a> Pageable for Commits<'a> {}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::api::projects::repository::commits::{Commits, CommitsBuilderError, CommitsOrder};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn commits_order_default() {
        assert_eq!(CommitsOrder::default(), CommitsOrder::Default);
    }

    #[test]
    fn commits_order_as_str() {
        let items = &[
            (CommitsOrder::Default, "default"),
            (CommitsOrder::Topo, "topo"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_necessary() {
        let err = Commits::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, CommitsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Commits::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("ref_name", "refs/tags/v1.0.0")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .ref_name("refs/tags/v1.0.0")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_since() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("since", "2021-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .since(Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_until() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("until", "2021-01-01T00:00:00Z")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .until(Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap())
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_path() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("path", "path/to/file")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .path("path/to/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("all", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .all(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_stats() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("with_stats", "false")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .with_stats(false)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_first_parent() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("first_parent", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .first_parent(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_order() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("order", "topo")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .order(CommitsOrder::Topo)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_trailers() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/repository/commits")
            .add_query_params(&[("trailers", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Commits::builder()
            .project("simple/project")
            .trailers(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
