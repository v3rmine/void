// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

#[derive(Debug, Clone)]
enum NameOrSearch<'a> {
    Name(Cow<'a, str>),
    Search(Cow<'a, str>),
}

/// States of environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentState {
    /// Environments that have been deployed and are available.
    Available,
    /// Environments that are being stopped.
    Stopping,
    /// Environments that have been stopped.
    Stopped,
}

impl EnvironmentState {
    fn as_str(self) -> &'static str {
        match self {
            EnvironmentState::Available => "available",
            EnvironmentState::Stopping => "stopping",
            EnvironmentState::Stopped => "stopped",
        }
    }
}

impl ParamValue<'static> for EnvironmentState {
    fn as_value(&self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for environments within a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Environments<'a> {
    /// The project to query for environments.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    #[builder(setter(name = "_name_or_search"), default, private)]
    name_or_search: Option<NameOrSearch<'a>>,
    /// Filter by the state of the environment.
    ///
    /// Note that even though the parameter is plural, it only supports a single value.
    #[builder(setter(into), default)]
    states: Option<EnvironmentState>,
}

impl<'a> Environments<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> EnvironmentsBuilder<'a> {
        EnvironmentsBuilder::default()
    }
}

impl<'a> EnvironmentsBuilder<'a> {
    /// Filter environments matching a name.
    ///
    /// Mutually exclusive with `search`.
    pub fn name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<Cow<'a, str>>,
    {
        self.name_or_search = Some(Some(NameOrSearch::Name(name.into())));
        self
    }

    /// Filter environments matching a search criteria.
    ///
    /// Mutually exclusive with `name`.
    pub fn search<S>(&mut self, search: S) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
    {
        self.name_or_search = Some(Some(NameOrSearch::Search(search.into())));
        self
    }
}

impl<'a> Endpoint for Environments<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/environments", self.project).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();

        params.push_opt("states", self.states);

        if let Some(name_or_search) = self.name_or_search.as_ref() {
            match name_or_search {
                NameOrSearch::Name(name) => {
                    params.push("name", name);
                },
                NameOrSearch::Search(search) => {
                    params.push("search", search);
                },
            }
        }

        params
    }
}

impl<'a> Pageable for Environments<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::environments::{
        EnvironmentState, Environments, EnvironmentsBuilderError,
    };
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn environment_state_as_str() {
        let items = &[
            (EnvironmentState::Available, "available"),
            (EnvironmentState::Stopped, "stopped"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = Environments::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, EnvironmentsBuilderError, "project");
    }

    #[test]
    fn project_is_sufficient() {
        Environments::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/1/environments")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Environments::builder().project(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_name() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/environments")
            .add_query_params(&[("name", "name")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Environments::builder()
            .project("simple/project")
            .name("name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_search() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/environments")
            .add_query_params(&[("search", "query")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Environments::builder()
            .project("simple/project")
            .search("query")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_state() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/environments")
            .add_query_params(&[("states", "available")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Environments::builder()
            .project("simple/project")
            .states(EnvironmentState::Available)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
