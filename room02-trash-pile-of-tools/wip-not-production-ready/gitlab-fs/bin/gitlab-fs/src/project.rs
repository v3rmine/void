use gitlab::{
    api::{self, projects::Projects, Query},
    Gitlab,
};
use log_utils::{debug, tracing};
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    #[serde(rename = "path_with_namespace")]
    pub full_path: String,
}

// Get all projects matching a query
#[tracing::instrument]
pub fn get_projects(client: &Gitlab, query: &str) -> Result<Vec<Project>> {
    let result = api::paged(
        Projects::builder()
            .search(query)
            .search_namespaces(true)
            .membership(true)
            .build()?,
        api::Pagination::All,
    )
    .query(client)?;

    debug!(projects=?result);

    Ok(result)
}
