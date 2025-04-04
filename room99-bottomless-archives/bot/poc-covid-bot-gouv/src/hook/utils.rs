use chrono::Local;
use sha2::{Digest, Sha256};

pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|_| "".to_owned())
}

pub fn write_file(path: &str, content: &str) {
    std::fs::write(path, content).unwrap();
}

pub fn sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(value.as_bytes());
    base64::encode(hasher.result())
}

pub fn time_now_formatted() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

#[derive(Debug)]
pub struct Date {
    pub year: String,
    pub month: String,
    pub day: String,
}

pub fn date_before_today(offset: i64) -> Date {
    let now = Local::now()
        .checked_sub_signed(chrono::Duration::days(offset))
        .unwrap();
    Date {
        year: now.format("%Y").to_string(),
        month: now.format("%m").to_string(),
        day: now.format("%d").to_string(),
    }
}
