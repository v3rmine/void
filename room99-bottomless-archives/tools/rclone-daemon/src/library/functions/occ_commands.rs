use super::{occ_command, occ_command_with_envs};
use std::process::Child;

pub fn execute_occ_scan(path: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command(format!("files:scan -qn --path {}", path).as_str())?.spawn()?)
}

pub fn execute_occ_scan_non_recursive(path: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command(format!("files:scan -qn --shallow --path {}", path).as_str())?.spawn()?)
}

pub fn create_occ_user(user: &str, pass: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command_with_envs(
        format!("user:add --password-from-env -qn -- {}", user).as_str(),
        &[("OC_PASS", pass)],
    )?
    .spawn()?)
}

pub fn delete_occ_user(user: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command(format!("user:delete -qn -- {}", user).as_str())?.spawn()?)
}

pub fn enable_occ_user(user: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command(format!("user:enable -qn --path {}", user).as_str())?.spawn()?)
}

pub fn disable_occ_user(user: &str) -> Result<Child, Box<dyn std::error::Error>> {
    Ok(occ_command(format!("user:disable -qn --path {}", user).as_str())?.spawn()?)
}
