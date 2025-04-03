use std::env::current_dir;
use std::fs::{create_dir, read, remove_dir_all, remove_file, File};
use std::io::Write;
use std::path::PathBuf;

use clap::ArgMatches;
use dirs::home_dir;
use indicatif::ProgressBar;
use walkdir::WalkDir;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::terminal::{alert, ask, error};
use crate::utils::types::FileContent;
use crate::INSTALL_DIR;

pub mod config;
pub mod terminal;
pub mod types;

pub fn prompt_overwrite_if_exist(path: &PathBuf, die_if_dont: bool) -> StandardResult<()> {
    let path_str = path.to_str().ok_or(BoilrError::StrError)?;

    if path.exists() {
        if ask(&format!(
            "File/Directory already exist at \"{}\" do you want to overwrite it?",
            path_str
        ))? {
            if path.is_dir() {
                alert(&format!("Overwriting dir recursively at \"{}\"", path_str));
                remove_dir_all(&path).map_err(|source| BoilrError::DeleteError {
                    source,
                    path: path.clone(),
                })?;
            } else {
                alert(&format!("Overwriting file at \"{}\"", path_str));
                remove_file(&path).map_err(|source| BoilrError::DeleteError {
                    source,
                    path: path.clone(),
                })?;
            }
        } else if die_if_dont {
            error("Please change output path if you do not want to overwrite it!");
            return Ok(());
        }
    }
    Ok(())
}

pub fn check_if_install_dir_exist() -> StandardResult<bool> {
    let path = home_dir()
        .ok_or(BoilrError::HomeDirNotFoundError)?
        .join(INSTALL_DIR);
    let exists = path.exists();

    if !exists {
        create_dir(&path).map_err(|source| BoilrError::WriteError {
            source,
            path: path.clone(),
        })?;
        create_dir(&path.join("templates")).map_err(|source| BoilrError::WriteError {
            source,
            path: path.join("templates"),
        })?;
        File::create(&path.join("templates.toml")).map_err(|source| BoilrError::WriteError {
            source,
            path: path.join("templates.toml"),
        })?;
    } else if !path.is_dir() {
        error(&format!(
            "Install dir (\"{}\") is not a directory",
            INSTALL_DIR
        ));
        return Err(BoilrError::NotADirectoryError { path });
    }

    Ok(exists)
}

pub fn to_output_path(args: &ArgMatches) -> StandardResult<PathBuf> {
    Ok(PathBuf::from(Option::unwrap_or(
        args.value_of("output"),
        current_dir()
            .map_err(|_| BoilrError::AccessCurrentDirError)?
            .to_str()
            .ok_or(BoilrError::StrError)?,
    )))
}

pub fn recursive_copy(from_path: &PathBuf, to_path: &PathBuf) -> StandardResult<()> {
    let walkdir_iter = WalkDir::new(from_path).follow_links(false).into_iter();

    let progress = ProgressBar::new_spinner();
    progress.set_message("Copying files and folders...");

    for entry in progress.wrap_iter(walkdir_iter) {
        match entry {
            Ok(entry) => {
                let write_path = to_path.join(entry.path().strip_prefix(from_path)?);

                if entry.path().is_file() {
                    let content = FileContent::Binary(read(entry.path()).map_err(|source| {
                        BoilrError::ReadError {
                            source,
                            path: entry.path().to_path_buf(),
                        }
                    })?);
                    create_and_write_file(&write_path, &content)?;
                } else {
                    create_dir(&write_path).map_err(|source| BoilrError::WriteError {
                        source,
                        path: write_path.clone(),
                    })?;
                }
            }
            Err(e) => return Err(e.into()),
        };
    }

    progress.finish_and_clear();

    Ok(())
}

pub fn create_and_write_file(path: &PathBuf, content: &FileContent) -> StandardResult<File> {
    let mut file = File::create(&path).map_err(|source| BoilrError::WriteError {
        source,
        path: path.clone(),
    })?;

    match content {
        FileContent::Text(content) => file.write_all(content.as_bytes()),
        FileContent::Binary(content) => file.write_all(&content),
    }
    .map_err(|source| BoilrError::WriteError {
        source,
        path: path.clone(),
    })?;

    Ok(file)
}
