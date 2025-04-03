use std::{collections::HashMap, fmt::Display};

use gitlab::{
    api::{self, projects::variables::ProjectVariables, Query},
    Gitlab,
};
use log_utils::{debug, tracing};
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub key: String,
    pub value: String,
    pub environment_scope: String,
}

/// Get the project env variables
#[tracing::instrument]
pub fn get_project_env(client: &Gitlab, project_id: u32) -> Result<Vec<Variable>> {
    let result = api::paged(
        ProjectVariables::builder()
            .project(project_id as u64)
            .build()?,
        api::Pagination::All,
    )
    .query(client)?;

    debug!(projects=?result);

    Ok(result)
}

/// Map the env variables to a list of (env_name, env_file)
#[tracing::instrument]
pub fn variables_to_env(variables: Result<Vec<Variable>>) -> Vec<(String, String)> {
    variables.map_or_else(
        |_| Vec::new(),
        |vars| {
            let envs = vars.iter().fold(
                HashMap::new() as HashMap<String, Vec<String>>,
                |mut acc, var| {
                    if let Some(env) = acc.get_mut(&var.environment_scope) {
                        env.push(var.to_string());
                    } else {
                        acc.insert(var.environment_scope.clone(), vec![var.to_string()]);
                    }
                    acc
                },
            );
            envs.into_iter()
                .map(|(k, mut v)| {
                    v.sort();
                    (k, v.join("\n"))
                })
                .collect::<Vec<_>>()
        },
    )
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
