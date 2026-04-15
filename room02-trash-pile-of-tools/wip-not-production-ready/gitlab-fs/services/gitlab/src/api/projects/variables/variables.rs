use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Filter parameters.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectVariablesFilter<'a> {
    /// Filter based on the environment scope.
    #[builder(setter(into), default)]
    environment_scope: Option<Cow<'a, str>>,
}

impl<'a> ProjectVariablesFilter<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectVariablesFilterBuilder<'a> {
        ProjectVariablesFilterBuilder::default()
    }

    pub(crate) fn add_query<'b>(&'b self, params: &mut FormParams<'b>) {
        params.push_opt("filter[environment_scope]", self.environment_scope.as_ref());
    }
}

/// Get the variable from a project.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct ProjectVariables<'a> {
    /// The project to get the variable from.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// Filter
    #[builder(default)]
    filter: Option<ProjectVariablesFilter<'a>>,
}

impl<'a> ProjectVariables<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectVariablesBuilder<'a> {
        ProjectVariablesBuilder::default()
    }
}

impl<'a> Endpoint for ProjectVariables<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/variables", self.project,).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = FormParams::default();

        if let Some(filter) = self.filter.as_ref() {
            filter.add_query(&mut params);
        }

        params.into_body()
    }
}

impl<'a> Pageable for ProjectVariables<'a> {}

#[cfg(test)]
mod tests {
    // TODO: Add some tests
}
