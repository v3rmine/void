use std::{
    fs::{remove_file, File, OpenOptions},
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use log::{debug, trace, tracing, warn};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::{prelude::*, Result};

const SLEEP_DURATION_BEFORE_RETRY_LOCK: Duration = Duration::from_millis(100);
const LOCKFILE_TIMEOUT: Duration = Duration::from_secs(5);

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    version: String,
    pub snapshots: Vec<SnapShot>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            version: CRATE_VERSION.to_string(),
            snapshots: Vec::new(),
        }
    }
}

impl Database {
    fn str_to_pathbuf(path: &str) -> Result<PathBuf> {
        let path = PathBuf::from_str(path).map_err(Error::InvalidPath)?;
        Ok(path)
    }

    #[tracing::instrument(level = "trace")]
    fn lock(options: OpenOptions, path: &PathBuf) -> Result<File> {
        let lockfile = PathBuf::from_str(&format!("{}.lock", path.to_str().unwrap()))
            .map_err(Error::InvalidPath)?;
        let instant = Instant::now();
        while lockfile.exists() {
            if instant.elapsed() > LOCKFILE_TIMEOUT {
                return Err(Error::LockFileTimeout(path.clone()));
            }
            sleep(SLEEP_DURATION_BEFORE_RETRY_LOCK);
        }

        {
            // Create lockfile and drop immediately the handle
            File::create(&lockfile).map_err(Error::FailedToOpenFile)?;
        }
        debug!("Created lockfile {:?}", &lockfile);
        let file = options.open(path).map_err(Error::FailedToOpenFile)?;
        let file_size = file
            .metadata()
            .map_err(Error::FailedToGetFileMetadata)?
            .len();
        debug!(
            "Opened {:?} for reading and writing, file size is {}",
            path, file_size,
        );

        Ok(file)
    }

    #[tracing::instrument(level = "trace")]
    fn unlock(path: &Path) -> Result<()> {
        if path.exists() {
            let lockfile = PathBuf::from_str(&format!("{}.lock", path.to_str().unwrap()))
                .map_err(Error::InvalidPath)?;
            remove_file(lockfile).map_err(Error::FailedToRemoveFile)?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug")]
    fn load_database(file: &File) -> Result<Self> {
        let file_size = file
            .metadata()
            .map_err(Error::FailedToGetFileMetadata)?
            .len();

        let mut result = if file_size == 0 {
            Database::default()
        } else {
            let mut reader = BufReader::new(file);
            let database = ciborium::de::from_reader::<Database, _>(&mut reader)?;
            tracing::debug!(
                "Deserialized database with {} snapshots",
                database.snapshots.len()
            );
            database
        };

        debug!("Loaded database with version {}", result.version);
        trace!("Loaded database from file \n{:#?}", result);

        if VersionReq::from_str(&format!(">{}", env!("CARGO_PKG_VERSION")))?
            .matches(&Version::from_str(&result.version)?)
        {
            warn!(
                "Database version mismatch, current version is {}, database version is {}",
                CRATE_VERSION, result.version
            );
            result.version = CRATE_VERSION.to_string();
        }

        Ok(result)
    }

    #[tracing::instrument(level = "debug")]
    fn write_self_to_file(&self, file: &File) -> Result<()> {
        let mut writer = BufWriter::new(file);
        debug!(
            "File size before write is {}",
            file.metadata()
                .map_err(Error::FailedToGetFileMetadata)?
                .len()
        );
        ciborium::ser::into_writer(&self, &mut writer)?;
        writer.flush().map_err(Error::FailedToWriteFile)?;
        debug!(
            "File size after write is {}",
            file.metadata()
                .map_err(Error::FailedToGetFileMetadata)?
                .len()
        );
        Ok(())
    }

    #[tracing::instrument]
    pub fn from_file(ipath: &str) -> Result<Self> {
        let path = Self::str_to_pathbuf(ipath)?;

        let mut options = OpenOptions::new();
        options.read(true);

        let file = Self::lock(options, &path)?;
        let result = Self::load_database(&file)?;
        Self::unlock(&path)?;

        Ok(result)
    }

    #[tracing::instrument]
    pub fn from_file_with_write(ipath: &str) -> Result<(Self, File, PathBuf)> {
        let path = Self::str_to_pathbuf(ipath)?;

        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        options.create(true);

        let mut file = Self::lock(options, &path)?;
        let result = Self::load_database(&file)?;

        // NOTE: We need to reset the file pointer to the beginning of the file to overwrite
        // SOURCE: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
        file.seek(SeekFrom::Start(0))
            .map_err(Error::FailedToSetFileCursor)?;

        Ok((result, file, path))
    }

    #[tracing::instrument(skip(self))]
    pub fn write_to_file(&self, path: &str) -> Result<()> {
        debug!(
            "Number of snapshot that will be written {}",
            self.snapshots.len()
        );
        let path = Self::str_to_pathbuf(path)?;

        let mut options = OpenOptions::new();
        options.write(true);
        options.truncate(true);
        options.create(true);

        let file = Self::lock(options, &path)?;
        debug!(
            "Number of snapshot that will be written {}",
            self.snapshots.len()
        );
        self.write_self_to_file(&file)?;
        Self::unlock(&path)?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn write_and_close_file(&self, file: File, path: &PathBuf) -> Result<()> {
        debug!(
            "Number of snapshot that will be written {}",
            self.snapshots.len()
        );
        self.write_self_to_file(&file)?;
        Self::unlock(path)?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn close_file(&self, path: &PathBuf) -> Result<()> {
        debug!(
            "Number of snapshot that would have been written {}",
            self.snapshots.len()
        );
        Self::unlock(path)?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn take_snapshot(&mut self, networks_to_ignore: &[&str]) -> Result<()> {
        self.snapshots.push(SnapShot::new(networks_to_ignore)?);
        debug!(
            "Number of snapshots after appending {}",
            self.snapshots.len()
        );

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn remove_older(&mut self, older_than_days: i64) -> Result<()> {
        let oldest_date = Utc::now()
            .checked_sub_signed(chrono::Duration::days(older_than_days))
            .ok_or(Error::OldestDateOverflow)?;
        self.snapshots.retain(|snap| snap.time > oldest_date);

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn get_cpu_usage(&self) -> Vec<(f64, DateTime<Utc>)> {
        let mut result: Vec<(f64, DateTime<Utc>)> = Vec::with_capacity(self.snapshots.len());
        let cpus_times = self
            .snapshots
            .iter()
            .map(|s| (s.get_cpu_time(), s.time))
            .collect::<Vec<_>>();

        for (idx, ((active, total), time)) in cpus_times.iter().enumerate() {
            let usage = active / total * 100.0;

            let idx = cpus_times.len() - idx - 1;
            debug!(idx, cpu_usage=?usage, time=?time);
            result.push((usage, *time));
        }

        debug!(cpu_usage_percentages = ?result);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_ram_usage(&self) -> Vec<((f64, f64), DateTime<Utc>)> {
        let result = self
            .snapshots
            .iter()
            .map(|s| (s.get_ram_usage(), s.time))
            .collect::<Vec<_>>();

        debug!(ram_usage_percentages = ?result);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_load(&self) -> Vec<((f64, f64, f64), DateTime<Utc>)> {
        let result = self
            .snapshots
            .iter()
            .map(|s| {
                let (one, five, fifteen) = s.get_load();
                let cpu_count = s.get_cpu_count() as f64;
                let to_percentage = |load| load / cpu_count * 100.0;
                (
                    (
                        to_percentage(one),
                        to_percentage(five),
                        to_percentage(fifteen),
                    ),
                    s.time,
                )
            })
            .collect::<Vec<_>>();

        debug!(load_avg = ?result);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_network(&self) -> Vec<((f64, f64), DateTime<Utc>)> {
        let result = self
            .snapshots
            .iter()
            .map(|s| {
                let (recv, sent) = s.get_network_usage();
                let to_mib = |bytes| bytes / 1024.0 / 1024.0;
                ((to_mib(recv), to_mib(sent)), s.time)
            })
            .collect::<Vec<_>>();

        debug!(network_usage = ?result);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_disks_speed_usage(&self) -> Vec<((f64, f64), DateTime<Utc>)> {
        let result = self
            .snapshots
            .iter()
            .map(|s| {
                let (read, written) = s.get_disk_speed_usage();
                let to_kib = |bytes: u64| (bytes / 1024) as f64;
                ((to_kib(read), to_kib(written)), s.time)
            })
            .collect::<Vec<_>>();

        debug!(disks_speed_usage = ?result);
        result
    }

    #[tracing::instrument(skip(self))]
    pub fn get_disk_memory_usage(&self) -> Vec<(f64, DateTime<Utc>)> {
        let result = self
            .snapshots
            .iter()
            .map(|s| {
                let usage = s
                    .get_disks_size_usage()
                    .into_iter()
                    .fold(0.0, |sum, (_label, usage)| sum + usage);
                let to_mib = |bytes| bytes / 1024.0 / 1024.0;
                (to_mib(usage), s.time)
            })
            .collect::<Vec<_>>();

        debug!(network_usage = ?result);
        result
    }
}
