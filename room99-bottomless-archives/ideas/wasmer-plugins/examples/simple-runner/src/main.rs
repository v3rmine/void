// Allow process::exit to keep the example simple
#![allow(clippy::exit)]

use std::{env, process};

use semver::{Version, VersionReq};
use simple_abi::GetVersionReq;
use wasm_plugins::runtime::{get_func_complex_without_args, instanciate_simple};

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

const RUNNER_VERSION: &str = "0.1.0";

fn main() {
    let mut args = env::args();
    let path = if let Some(path) = args.nth(1) {
        path
    } else {
        println!("Usage: ./simple-runner PLUGIN_PATH");
        process::exit(1);
    };

    if let Err(e) = run_plugin(&path) {
        eprintln!("Error when running the plugin: {}", e);
        process::exit(2);
    };
}

fn run_plugin(path: &str) -> StdResult<()> {
    let instance = instanciate_simple(path)?;
    let get_plugin_requirements = get_func_complex_without_args::<GetVersionReq>(&instance)?;

    let plugin_requirements = get_plugin_requirements()?;

    println!("Runner version is {}", RUNNER_VERSION);
    println!("Plugin requirements are {}", plugin_requirements);

    let semver_req = VersionReq::parse(&plugin_requirements)?;
    let runner_version = Version::parse(RUNNER_VERSION)?;

    if semver_req.matches(&runner_version) {
        println!("The runner support this plugin");
    } else {
        println!("The runner does not support this plugin");
    }

    Ok(())
}
