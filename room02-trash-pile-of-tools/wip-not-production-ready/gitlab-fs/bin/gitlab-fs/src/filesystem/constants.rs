use std::time::Duration;

/// Documentation for LIBC Constants <https://www.gnu.org/software/libc/manual/html_node/Error-Codes.html>
pub use libc::*;

/// The time to live of the filesystem responses (E.g. `getattr`)
pub const TTL: Duration = Duration::from_millis(1000);
