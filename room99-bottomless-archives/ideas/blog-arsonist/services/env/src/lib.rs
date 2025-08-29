use std::{env::VarError, ffi::OsStr};

use dotenvy::dotenv;
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

pub fn setup_env() -> Result<(), Error> {
    dotenv()?;
    Ok(())
}

#[tracing::instrument]
pub fn var_not_empty<K>(key: K) -> Result<String, Error>
where
    K: AsRef<OsStr>,
    K: std::fmt::Display + std::fmt::Debug,
{
    let val = std::env::var(&key)?;

    if val.is_empty() {
        Err(Error::EmptyVar(key.to_string()))
    } else {
        Ok(val)
    }
}
