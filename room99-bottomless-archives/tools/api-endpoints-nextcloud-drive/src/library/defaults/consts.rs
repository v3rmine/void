#![allow(dead_code)]
pub const ADDR: &str = "127.0.0.1:3001";
pub const LOG_LEVEL: &str = "INFO";
#[cfg(target_os = "windows")]
pub const EXECUTABLE_PATH: &str = "\\bin";
#[cfg(not(target_os = "windows"))]
pub const EXECUTABLE_PATH: &str = "/bin";
#[cfg(target_os = "windows")]
pub const CERTIFICATOR: &str = "\\gsuite-cert.exe";
#[cfg(not(target_os = "windows"))]
pub const CERTIFICATOR: &str = "/gsuite-cert";
#[cfg(target_os = "windows")]
pub const NEXT: &str = "\\nextcloud-api-rs.exe";
#[cfg(not(target_os = "windows"))]
pub const NEXT: &str = "/nextcloud-api-rs";
