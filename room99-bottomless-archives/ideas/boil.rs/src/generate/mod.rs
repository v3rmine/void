use std::path::PathBuf;

use clap::ArgMatches;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::terminal::{error, notify};
use crate::utils::to_output_path;
use crate::TEMPLATE_CONFIG_NAME;

mod config;
mod output;
mod parser;
mod plugins;
mod scanner;

pub fn generate(args: &ArgMatches) -> StandardResult<()> {
    // * Parsing cli args
    let template_path = PathBuf::from(
        args.value_of("template")
            .ok_or(BoilrError::ArgNotFoundError)?,
    );
    let template_path_str = template_path.to_str().ok_or(BoilrError::StrError)?;

    let output_path = to_output_path(args)?;
    if !output_path.is_dir() {
        error("Output path is not a directory!");
        return Err(BoilrError::NotADirectoryError { path: output_path });
    }

    let name = args.value_of("name").ok_or(BoilrError::ArgNotFoundError)?;

    // * Generate placeholders from project.toml
    let config = config::parse_config(&template_path.join(TEMPLATE_CONFIG_NAME))?;

    // * Scanning files and folders in template dir
    let (folder_entries, file_entries) = scanner::scan_dir(&template_path)?;
    notify(&format!(
        "Scanned files and folders in \"{}\"",
        template_path_str
    ));

    // * Parse files using Tera
    let process_files = parser::process_files(&template_path, file_entries, &config)?;
    notify(&format!("Parsed files in \"{}\"", template_path_str));

    // * Reconstruct template in output
    let full_output_path = output_path.join(name);
    let full_output_path_str = full_output_path.to_str().ok_or(BoilrError::StrError)?;

    output::reconstruct(&template_path, &full_output_path, &folder_entries)?;
    notify(&format!(
        "Reconstructed template directories at \"{}\"",
        full_output_path_str
    ));

    output::write(&full_output_path, &process_files)?;

    notify(&format!(
        "Successfully created template at \"{}\"",
        full_output_path_str
    ));

    Ok(())
}
