use std::fs::remove_dir_all;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::config::ConfigIO;
use crate::utils::terminal::{alert, notify};

pub fn uninstall(args: &ArgMatches) -> StandardResult<()> {
    let template_name = args.value_of("name").ok_or(BoilrError::ArgNotFoundError)?;

    let mut io = ConfigIO::new()?;

    let template_index = io.find_index(|t| t.name == template_name);

    if let Some(template_index) = template_index {
        let template_path = PathBuf::from(io.config.templates.remove(template_index).path);
        remove_dir_all(&template_path).map_err(|source| BoilrError::WriteError {
            source,
            path: template_path,
        })?;
        io.write_config()?;

        notify(&["Template '", template_name, "' successfully uninstalled"].concat());
    } else {
        alert("Cannot uninstall, config not found!");
    }

    Ok(())
}
