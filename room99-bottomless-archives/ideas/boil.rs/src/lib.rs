#[macro_use]
extern crate clap;

pub mod errors;
// subcommands
pub mod generate;
pub mod install;
pub mod list;
pub mod new;
pub mod uninstall;
pub mod utils;

// CONSTS
pub const DEFAULT_ASK: bool = false;
// Install dir is in the $HOME user directory
pub const INSTALL_DIR: &str = ".boilrs";
pub const CONFIG_FILE_NAME: &str = "templates.toml";
pub const TEMPLATE_IGNORE_FILE: &str = ".ignore";
pub const TEMPLATE_DIR_NAME: &str = "template";
pub const TEMPLATE_CONFIG_NAME: &str = "project.toml";
