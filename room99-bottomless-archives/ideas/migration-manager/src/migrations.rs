use std::{fs, path::PathBuf};

use chrono::{DateTime, FixedOffset};
use rayon::iter::*;
use tracing::trace;
use walkdir::WalkDir;

use crate::{cli::MigrationType, Result};

const CHRONO_FORMAT: &str = "%Y_%m_%d_%H_%M_%S";

#[derive(Debug)]
struct Migration {
    pub name: String,
    pub path: PathBuf,
    pub date: DateTime<FixedOffset>,
    pub m_type: MigrationType,
}

#[tracing::instrument]
pub fn list_migrations(folder: &str) -> Result<()> {
    let entries = WalkDir::new(folder)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();

    entries
        .par_iter()
        .map(|entry| -> Result<Migration> {
            let (date, name) = entry
                .file_name()
                .to_str()
                .ok_or_else(|| eyre::eyre!("Cannot convert the filename to string"))?
                .split_once('-')
                .ok_or_else(|| eyre::eyre!("Cannot find the parts of the migration name"))?;
            let date =
                DateTime::parse_from_str(&format!("{date} +0000"), &format!("{CHRONO_FORMAT} %z"))?;
            let name = name.replace('_', " ");
            let path = entry.path().to_path_buf();

            let m_type = if path.join("up.sh").exists() && path.join("down.sh").exists() {
                MigrationType::Shell
            } else {
                MigrationType::Sql
            };

            trace!("Found migration ({m_type}) => {name} : {date}");

            Ok(Migration {
                name,
                date,
                path,
                m_type,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}

#[tracing::instrument]
pub fn create_migration(folder: &str, name: &str, m_type: &MigrationType) -> Result<()> {
    let migration_full_name = format!(
        "{}-{}",
        chrono::Utc::now().format(CHRONO_FORMAT),
        name.to_lowercase().replace(' ', "_")
    );
    let migration_path = PathBuf::from(folder).join(migration_full_name);

    fs::create_dir_all(&migration_path)?;

    match m_type {
        MigrationType::Shell => {
            fs::File::create(&migration_path.join("up.sh"))?;
            fs::File::create(&migration_path.join("down.sh"))?;
        }
        MigrationType::Sql => {
            fs::File::create(&migration_path.join("up.sql"))?;
            fs::File::create(&migration_path.join("down.sql"))?;
        }
    }

    Ok(())
}
