use std::collections::HashMap;
use std::path::PathBuf;

use console::style;
use dialoguer::{Confirm, Input, Select};
use tera::Value;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::terminal::alert;

pub fn parse_config(path: &PathBuf) -> StandardResult<HashMap<String, Value>> {
    let toml: toml::Value = toml::from_str(&std::fs::read_to_string(&path).map_err(|source| {
        BoilrError::ReadError {
            source,
            path: path.clone(),
        }
    })?)
    .map_err(|source| BoilrError::TomlDeserializeError {
        source,
        path: path.clone(),
    })?;

    let mut config: HashMap<String, Value> = HashMap::new();

    for (key, val) in toml
        .as_table()
        .ok_or_else(|| BoilrError::UnspecifiedError(Some("Cannot parse config file".into())))?
    {
        match val {
            toml::Value::Array(vals) => {
                config.insert(key.clone(), Value::String(ask_within_array(key, vals)?))
            }
            toml::Value::String(el) => config.insert(
                key.clone(),
                Value::String(ask_with_default_string(key, el)?),
            ),
            toml::Value::Boolean(b) => {
                config.insert(key.clone(), Value::Bool(ask_confirmation(key, b)?))
            }
            _ => {
                alert(&format!(
                    "Unsupported variable type in the configuration: \"{}\" with value \"{}\"",
                    key,
                    val.to_string()
                ));
                None
            }
        };
    }

    Ok(config)
}

fn ask_with_default_string(key: &str, default: &str) -> StandardResult<String> {
    Input::<String>::new()
        .default(default.to_string())
        .show_default(false)
        .with_prompt(format!(
            "{} {} \"{}\" {}",
            style("[?]").bold().cyan(),
            style("Please choose a value for").bold(),
            style(key).bold(),
            style(format!("[default: \"{}\"]", default)).cyan()
        ))
        .interact()
        .map_err(|source| BoilrError::TerminalError { source })
}

fn ask_within_array(key: &str, arr: &[toml::Value]) -> StandardResult<String> {
    let arr = arr
        .iter()
        .map(|el| {
            el.as_str().ok_or_else(|| {
                BoilrError::UnspecifiedError(Some(
                    "Internal error while prompting for array element".into(),
                ))
            })
        })
        .collect::<StandardResult<Vec<&str>>>()?;
    Ok(arr
        .get(
            Select::new()
                .default(0)
                .with_prompt(format!(
                    "{} {} \"{}\" {}",
                    style("[?]").bold().cyan(),
                    style("Please choose an option for").bold(),
                    style(key).bold(),
                    style(format!(
                        "[default: \"{}\"]",
                        arr.first()
                            .ok_or_else(|| BoilrError::UnspecifiedError(Some(
                                "Array in config is empty".into()
                            )))?
                    ))
                    .cyan()
                ))
                .items(&arr)
                .interact()
                .map_err(|source| BoilrError::TerminalError { source })?,
        )
        .ok_or_else(|| {
            BoilrError::UnspecifiedError(Some(
                "Internal error while prompting for array element".into(),
            ))
        })?
        .to_string())
}

fn ask_confirmation(key: &str, default: &bool) -> StandardResult<bool> {
    Confirm::new()
        .default(*default)
        .show_default(false)
        .with_prompt(format!(
            "{} {} \"{}\" {}",
            style("[?]").bold().cyan(),
            style("Please choose (true/false) for").bold(),
            style(key).bold(),
            style(format!("[default: \"{}\"]", default)).cyan()
        ))
        .interact()
        .map_err(|source| BoilrError::TerminalError { source })
}
