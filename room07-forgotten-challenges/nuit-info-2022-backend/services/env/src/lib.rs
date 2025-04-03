//! Provide utilities for working with the environment variables.

use std::{env::VarError, ffi::OsStr, path::Path};

use dotenvy::{dotenv, from_path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("environment variable `{0}` is empty")]
    EmptyVar(String),
    #[error("environment variable `{0}` is not set")]
    VarNotSet(#[from] VarError),
    #[error("failed to load .env file {0}")]
    DotEnv(#[from] dotenvy::Error),
}

/// Load .env and ignore any errors
pub fn setup_env() {
    dotenv().ok();
}

/// Load env from a specific path and ignore any errors
pub fn setup_env_with_path(path: &Path) {
    from_path(path).ok();
}

/// Check if an environment variable exist and is not empty
#[tracing::instrument(level = "debug")]
pub fn var_not_empty<K>(key: K) -> Result<String, Error>
where
    K: AsRef<OsStr>,
    K: std::fmt::Display + std::fmt::Debug,
{
    let val = std::env::var(&key)?;
    tracing::debug!(value = val);

    if val.is_empty() {
        Err(Error::EmptyVar(key.to_string()))
    } else {
        Ok(val)
    }
}
