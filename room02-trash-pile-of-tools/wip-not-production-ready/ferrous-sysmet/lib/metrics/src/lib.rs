#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "thresholds")]
pub mod thresholds;

pub mod errors;
pub mod psutil;
pub mod snapshot;

pub mod prelude {
    #[cfg(feature = "database")]
    pub use super::database::Database;
    #[cfg(feature = "thresholds")]
    pub use super::thresholds::*;

    pub use super::errors::Error;
    pub use super::snapshot::SnapShot;

    pub fn get_hostname() -> String {
        ::psutil::host::info().hostname().to_string()
    }
}

pub(crate) use errors::Result;
