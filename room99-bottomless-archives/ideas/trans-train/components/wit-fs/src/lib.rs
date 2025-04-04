wit_bindgen::generate!("fs" in "../../wit/fs.wit");

use wit_log::{trace, warn};

use std::{io::{Read, Write}, fmt::Display};
use thiserror::Error;

#[derive(Debug)]
pub struct TransformerFile {
    handle: fs::FileHandle,
    path: String,
}

impl Display for TransformerFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("interacting with the file")]
    Io,
    #[error("access or operation to the file is denied")]
    PermissionDenied
}

impl From<fs::Error> for Error {
    fn from(value: fs::Error) -> Self {
        match value {
            fs::Error::Io => Error::Io,
            fs::Error::PermissionDenied => Error::PermissionDenied,
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        Self::new(std::io::ErrorKind::Other, value)
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}

impl TransformerFile {
    pub fn open_input(path: &str) -> Result<Self, Error> {
        trace!("opened input {path}");
        Ok(Self{ 
            handle: fs::open_input(path)?, 
            path: path.to_string(),
        })
    }
    pub fn open_output(path: &str) -> Result<Self, Error> {
        trace!("opened output {path}");
        Ok(Self {
            handle: fs::open_output(path)?, 
            path: path.to_string()
        })
    }
}

impl Drop for TransformerFile {
    fn drop(&mut self) {
        trace!("closing {self}");
        if fs::close(&self.handle).is_some() {
            warn!("failed to close {self} correctly");
        }
    }
}

impl Read for TransformerFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        trace!("reading from {self}");
        let (content, len) = fs::read(&self.handle).map_err(Error::from)?;
        trace!("read {len} bytes from {self}");
        buf.copy_from_slice(&content);
        Ok(len as usize)
    }
}

impl Write for TransformerFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        trace!("writing in {self}");
        let len = fs::write(&self.handle, buf).map_err(Error::from)?;
        trace!("wrote {len} bytes in {self}");
        Ok(len as usize)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        trace!("flushed {self}");
        fs::flush(&self.handle).ok_or(Error::Io)?;
        Ok(())
    }
}
