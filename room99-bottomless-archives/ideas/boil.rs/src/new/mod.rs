use std::fs::{create_dir, File};

use clap::ArgMatches;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::terminal::{error, notify};
use crate::utils::{prompt_overwrite_if_exist, to_output_path};
use crate::{TEMPLATE_CONFIG_NAME, TEMPLATE_DIR_NAME, TEMPLATE_IGNORE_FILE};

pub fn new(args: &ArgMatches) -> StandardResult<()> {
    let output_path = to_output_path(args)?;

    let template_name = args.value_of("name").ok_or(BoilrError::ArgNotFoundError)?;

    if !output_path.is_dir() {
        error("Output path is not a directory!");
        return Err(BoilrError::NotADirectoryError { path: output_path });
    }

    let full_output_path = output_path.join(template_name);

    prompt_overwrite_if_exist(&full_output_path, true)?;

    create_dir(&full_output_path).map_err(|source| BoilrError::WriteError {
        source,
        path: full_output_path.clone(),
    })?;
    create_dir(&full_output_path.join(TEMPLATE_DIR_NAME)).map_err(|source| {
        BoilrError::WriteError {
            source,
            path: full_output_path.join(TEMPLATE_DIR_NAME),
        }
    })?;
    File::create(&full_output_path.join(TEMPLATE_IGNORE_FILE)).map_err(|source| {
        BoilrError::WriteError {
            source,
            path: full_output_path.join(TEMPLATE_IGNORE_FILE),
        }
    })?;
    File::create(&full_output_path.join(TEMPLATE_CONFIG_NAME)).map_err(|source| {
        BoilrError::WriteError {
            source,
            path: full_output_path.join(TEMPLATE_CONFIG_NAME),
        }
    })?;

    notify(
        &[
            "New blank template created at '",
            full_output_path.to_str().ok_or(BoilrError::StrError)?,
            "'",
        ]
        .concat(),
    );

    Ok(())
}
