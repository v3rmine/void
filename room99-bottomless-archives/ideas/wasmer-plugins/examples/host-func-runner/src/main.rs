// Allow process::exit to keep the example simple
#![allow(clippy::exit)]

use std::{env, process};

use host_func_abi::GetVersionReq;
use wasm_plugins::runtime::{get_func_complex_without_args, InstanceBuilder};
use wasmer::{imports, Function, Memory, Store};

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    let store = Store::default();
    let get_page = Function::new_native(&store, |ptr: i32, len: i32| {
        println!("IT WORK {}", ptr);
        len - 1
    });

    let instance = InstanceBuilder::builder()
        .wasm_path(path)
        .import_object(imports! {
            "env" => {
                "_get_page"  => get_page,
            }
        })
        .build()
        .finalize()?;

    dbg!(&instance.exports);

    let get_plugin_requirements = get_func_complex_without_args::<GetVersionReq>(&instance)?;

    let plugin_requirements = get_plugin_requirements()?;
    println!("Plugin requirements are {}", plugin_requirements);

    Ok(())
}
