use std::process::ExitCode;

use illiterate::{
    cli::{self, Commands},
    commands,
};
use tracing::trace;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<ExitCode> {
    let (source_dir, cli, command) = cli::Cli::build()?;
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&cli.log_level));
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(filter)
        .init();
    trace!(source = source_dir, config =? &cli, "started app");

    match command.unwrap_or(Commands::Check) {
        Commands::Check => {
            Ok(commands::check::check(&source_dir, &cli))
        }
        Commands::Tangle { outdir, dry_run } => {
            Ok(commands::tangle::tangle(
                &source_dir,
                &cli,
                &outdir,
                dry_run,
            ))
        }
        Commands::Watch { outdir, poll } => {
            commands::watch::watch(
                &source_dir,
                &cli,
                &outdir,
                poll,
            );
            Ok(ExitCode::SUCCESS)
        }
    }
}
