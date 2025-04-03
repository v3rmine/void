use std::{env, path::PathBuf};

use crate::Result;

#[cfg(not(target_os = "windows"))]
const TEMP_DIR_ENV: &str = "TMP";
#[cfg(target_os = "windows")]
const TEMP_DIR_ENV: &str = "TEMP";
const HOME_DIR_ENV: &str = "HOME";

pub fn get_temp_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(format!(
        "{}/.migration-manager-temp",
        env::var(TEMP_DIR_ENV)
            .unwrap_or_else(|_| env::var(HOME_DIR_ENV).unwrap_or_else(|_| ".".to_string()))
    )))
}
