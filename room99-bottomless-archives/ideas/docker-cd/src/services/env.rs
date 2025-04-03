use std::{ffi::OsStr, fs};

use eyre::Result;

const DEFAULT_TIMEZONE: &str = "Etc/UTC";

pub fn setup_env() -> Result<()> {
    let timezone = var_not_empty("TZ").unwrap_or_else(|_| {
        fs::read_to_string("/etc/timezone")
            .map_or_else(|_| DEFAULT_TIMEZONE.to_string(), |s| s.trim().to_string())
    });
    std::env::set_var("TZ", timezone);

    Ok(())
}

pub fn var_not_empty<K>(key: K) -> Result<String>
where
    K: AsRef<OsStr>,
    K: std::fmt::Display,
{
    let val = std::env::var(&key)?;

    if val.is_empty() {
        Err(eyre::eyre!("{key} is empty"))
    } else {
        Ok(val)
    }
}
