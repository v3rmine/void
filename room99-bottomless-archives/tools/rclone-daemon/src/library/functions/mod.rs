#![allow(dead_code)]
use std::process::Command;

pub const TEMPLATE_CONFIG: &str = include_str!("../../templates/template.config");
pub const TEMPLATE_SERVICE: &str = include_str!("../../templates/template.service");

macro_rules! replace {
    ($content:expr, $($i:ident=$v:expr),*) => {
        |c: String, replacements: Vec<(String, String)>| -> String {
            let mut result: String = c;
            for rep in replacements {
                result = result.replace(rep.0.as_str(), rep.1.as_str());
            }
            result
        }($content, vec![$((format!("{{{}}}", stringify!($i)), $v)),*])
    };
}

/* rclone.conf */
pub mod config_management;
pub use config_management::{RcloneConfig, RcloneParser};

/* service management */
pub mod service_management;

/* commands */
pub mod occ_commands;

/* Base command */
pub fn occ_command(command: &str) -> Result<Command, Box<dyn std::error::Error>> {
    Ok(execute_command_as(
        "www-data",
        format!(
            "php {}/occ {}",
            super::defaults::NEXTCLOUD_DIRECTORY,
            command
        )
        .as_str(),
    ))
}
pub fn occ_command_with_envs(
    command: &str,
    envs: &[(&str, &str)],
) -> Result<Command, Box<dyn std::error::Error>> {
    let envs_formatted = |envs_list: &[(&str, &str)]| -> String {
        let mut result = String::new();
        for (k, v) in envs_list {
            result = format!("{} {}='{}'", result, k, v);
        }
        result
    }(envs);
    let mut com = Command::new("/usr/bin/sudo");
    com.args(&[
        "su",
        "www-data",
        "-s",
        "/bin/sh",
        "-c",
        format!(
            "{} php {}/occ {}",
            envs_formatted,
            super::defaults::NEXTCLOUD_DIRECTORY,
            command
        )
        .as_str(),
    ]);
    Ok(com)
}
pub fn execute_command(command: &str) -> Command {
    let mut com = Command::new("/usr/bin/sudo");
    com.args(&["su", "-s", "/bin/sh", "-c", format!("{}", command).as_str()]);
    println!("{:?}", com);
    com
}
fn execute_command_as(user: &str, command: &str) -> Command {
    let mut com = Command::new("/usr/bin/sudo");
    com.args(&[
        "su",
        user,
        "-s",
        "/bin/sh",
        "-c",
        format!("{}", command).as_str(),
    ]);
    println!("{:?}", com);
    com
}
fn execute_command_as_with_envs(user: &str, command: &str, envs: &[(&str, &str)]) -> Command {
    let mut com = Command::new("/usr/bin/sudo");
    com.args(&[
        "su",
        user,
        "-s",
        "/bin/sh",
        "-c",
        format!("{}", command).as_str(),
    ]);
    for (ref k, ref v) in envs {
        com.env(k, v);
    }
    println!("{:?}", com);
    com
}

#[derive(Debug, Clone, Copy)]
pub struct Mount<'a> {
    pub id: &'a str,
    pub path: &'a str,
    pub mounted: bool,
    pub service_name: &'a str,
}
