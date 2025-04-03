use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use indicatif::ProgressBar;
use tera::{Context, Result, Tera, Value};
use walkdir::DirEntry;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::terminal::error;
use crate::utils::types::FileContent;

// * Plugins
#[cfg(feature = "case_mod")]
use super::plugins::case_mod;

pub type TeraFilter<'a> =
    &'a (dyn (Fn(&Value, &HashMap<String, Value>) -> Result<Value>) + Send + Sync);

pub fn process_files(
    template_path: &PathBuf,
    files: Vec<DirEntry>,
    config: &HashMap<String, Value>,
) -> StandardResult<Vec<(PathBuf, FileContent)>> {
    let mut processed_files = Vec::new();

    let progress = ProgressBar::new_spinner();
    progress.set_message("[2/4] Parsing files in template...");

    for file in progress.wrap_iter(files.iter()) {
        match std::fs::read_to_string(file.path()) {
            Ok(content) => {
                let processed = parse(&content, config)?;
                processed_files.push((
                    file.path()
                        .strip_prefix(template_path.join("template"))?
                        .to_path_buf(),
                    FileContent::Text(processed),
                ))
            }
            _ => processed_files.push((
                file.path()
                    .strip_prefix(template_path.join("template"))?
                    .to_path_buf(),
                FileContent::Binary(std::fs::read(file.path()).map_err(|source| {
                    BoilrError::ReadError {
                        source,
                        path: file.path().to_path_buf(),
                    }
                })?),
            )),
        }
    }

    progress.finish_and_clear();
    Ok(processed_files)
}

fn parse(text: &str, config: &HashMap<String, Value>) -> StandardResult<String> {
    let mut tera = Tera::default();
    let mut context = Context::new();
    for (key, value) in config {
        context.insert(key, &value);
    }

    for (filter_name, filter) in filters_plugins() {
        tera.register_filter(filter_name, filter);
    }

    Ok(tera.render_str(text, &context).map_err(|e| {
        error(&format!("Error while rendering template: {:?}", e.source()));
        e
    })?)
}

#[allow(unused_mut)]
#[allow(clippy::let_and_return)]
#[allow(unused_variables)]
fn filters_plugins<'a>() -> Vec<(&'static str, TeraFilter<'a>)> {
    let mut result = Vec::new();
    #[cfg(feature = "case_mod")]
    result.extend(case_mod::all());
    result
}
