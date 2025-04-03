#![forbid(unsafe_code)]

use std::env::{set_var, var};

use clap::{ArgAction, Parser};
use once_cell::sync::Lazy;
use sis_server::{run_server, Result};

// NOTE: Use HOST and PORT env variables as defaults (runtime)
static DEFAULT_ADDRESS: Lazy<String> = Lazy::new(|| {
    format!(
        "{}:{}",
        var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        var("PORT").unwrap_or_else(|_| "8080".to_string())
    )
});

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_name = "LISTENING ADDRESS", default_value = DEFAULT_ADDRESS.as_str())]
    address: String,
    #[clap(short, long = "verbose", action = ArgAction::Count)]
    verbosity: u8,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env::setup_env();

    let app = Cli::parse();

    if app.verbosity > 2 {
        set_var("LOG_LEVEL", "trace");
    } else if app.verbosity == 2 {
        set_var("LOG_LEVEL", "debug");
    } else if app.verbosity == 1 {
        set_var("LOG_LEVEL", "info");
    }

    let _logfiles_writer_handle = logging::setup_logger_with_logfiles(env!("CARGO_PKG_NAME"));

    run_server(app.address.parse()?).await?;

    Ok(())
}
