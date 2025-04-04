mod env;
pub use env::*;
mod serial;
pub use serial::*;
mod macros;
pub use macros::*;
pub mod uri;

pub fn sleep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time));
}