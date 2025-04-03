#![allow(dead_code)]
use std::io;

use clap::Parser;
use cli::Command;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::cli::Cli;

mod cli;
mod database;
mod migrations;
mod os;

pub type Result<T> = eyre::Result<T>;

const DEFAULT_LOG_LEVEL: &str = "trace";

fn main() -> Result<()> {
    // Setup error handling
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(DEFAULT_LOG_LEVEL.parse()?)
                .with_env_var("LOG_LEVEL")
                .from_env()?,
        )
        .with({
            let layer = fmt::layer().with_writer(io::stdout).without_time();
            // NOTE: Print the origin of the log only when in debug mode.
            #[cfg(not(debug_assertions))]
            let layer = layer.with_target(false).compact();
            #[cfg(debug_assertions)]
            let layer = layer.pretty();

            layer
        })
        .init();

    match Cli::parse() {
        Cli {
            migration_folder,
            command:
                Command::Create {
                    name,
                    migration_type,
                },
        } => {
            migrations::create_migration(&migration_folder, &name, &migration_type)?;
        }
        Cli {
            migration_folder,
            command: Command::Status { .. },
        } => {
            migrations::list_migrations(&migration_folder)?;
        }
        x => {
            dbg!(x);
        }
    }

    Ok(())
}
