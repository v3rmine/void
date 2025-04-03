use std::fs::create_dir;
use std::path::PathBuf;

use indicatif::ProgressBar;
use walkdir::DirEntry;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::types::FileContent;
use crate::utils::{create_and_write_file, prompt_overwrite_if_exist};
use crate::TEMPLATE_DIR_NAME;

pub fn reconstruct(
    from_path: &PathBuf,
    path: &PathBuf,
    folders: &[DirEntry],
) -> StandardResult<()> {
    prompt_overwrite_if_exist(&path, true)?;
    create_dir(path).map_err(|source| BoilrError::WriteError {
        source,
        path: path.clone(),
    })?;

    let progress = ProgressBar::new_spinner();
    progress.set_message("[3/4] Reconstructing template directories...");

    for folder in progress.wrap_iter(folders.iter()) {
        let new_path = path.join(
            folder
                .path()
                .strip_prefix(from_path.join(TEMPLATE_DIR_NAME))?,
        );
        create_dir(&new_path).map_err(|source| BoilrError::WriteError {
            source,
            path: new_path.to_path_buf(),
        })?;
    }

    progress.finish_and_clear();
    Ok(())
}

pub fn write(path: &PathBuf, files: &[(PathBuf, FileContent)]) -> StandardResult<()> {
    let progress = ProgressBar::new_spinner();
    progress.set_message("[4/4] Writing files to output...");

    for (file_path, file_content) in progress.wrap_iter(files.iter()) {
        let path = path.join(file_path);
        create_and_write_file(&path, file_content)?;
    }

    progress.finish_and_clear();
    Ok(())
}
