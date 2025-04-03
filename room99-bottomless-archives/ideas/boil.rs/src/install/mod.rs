use std::env::current_dir;
use std::path::PathBuf;

use clap::ArgMatches;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::config::ConfigIO;
use crate::utils::terminal::{alert, error, notify};
use crate::utils::{prompt_overwrite_if_exist, recursive_copy};
use crate::TEMPLATE_CONFIG_NAME;

pub fn install(args: &ArgMatches) -> StandardResult<()> {
    // Template from
    let mut template_path = PathBuf::from(Option::unwrap_or(
        args.value_of("path"),
        current_dir()
            .map_err(|_| BoilrError::AccessCurrentDirError)?
            .to_str()
            .ok_or(BoilrError::StrError)?,
    ));

    let template_name = args.value_of("name").ok_or(BoilrError::ArgNotFoundError)?;

    if !template_path.join(TEMPLATE_CONFIG_NAME).is_file() {
        template_path = template_path.join(template_name);
    }

    if !template_path.join(TEMPLATE_CONFIG_NAME).is_file() {
        error(
            &[
                "Cannot find any valid template at ",
                template_path.to_str().ok_or(BoilrError::StrError)?,
            ]
            .concat(),
        );
        return Ok(());
    }

    // Template to
    let mut io = ConfigIO::new()?;

    let templates = &io.config.templates;

    if let Some(template) = templates.iter().find(|t| t.name == template_name) {
        alert("This template is already installed");
        prompt_overwrite_if_exist(
            &io.dir
                .clone()
                .ok_or(BoilrError::UnspecifiedError(None))?
                .join(&template.path),
            false,
        )?;
        io.retain_templates(|t| t.name != template_name);
    }

    // Recursive copy directory
    let target_path = io
        .dir
        .clone()
        .ok_or(BoilrError::UnspecifiedError(None))?
        .join("templates")
        .join(template_name);

    recursive_copy(&template_path, &target_path).map_err(|source| BoilrError::CopyError {
        source: Box::new(source),
        from_path: template_path.clone(),
        to_path: target_path.clone(),
    })?;

    // Write templates.toml

    io.push_template(template_name, &target_path)?;

    io.write_config()?;

    notify(&["Template '", template_name, "' successfully installed"].concat());

    Ok(())
}
