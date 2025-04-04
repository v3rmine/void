use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Layer, util::SubscriberInitExt};
use wasm_runner::WasiTransformerRuntime;

mod config;
mod transformers;
mod wasm_runner;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::Registry::default()
        .with(tracing_utils::with_env().and_then(tracing_utils::with_pretty()))
        .init();
    
    // https://github.com/bytecodealliance/wasmtime/blob/main/crates/test-programs/tests/wasi-preview2-components.rs
    // let config = std::fs::read_to_string("config.toml")?;
    // let config = toml::from_str::<Config>(&config)?;
    // dbg!(&config);

    let mut runtime = WasiTransformerRuntime::default();
    runtime.register_transformer(
        "hello_world",
        "./target/components/wasm32-unknown-unknown/debug/hello_world_transformer.wasm",
        vec![],
    )?;
    runtime.call("hello_world").unwrap();

    Ok(())
}
