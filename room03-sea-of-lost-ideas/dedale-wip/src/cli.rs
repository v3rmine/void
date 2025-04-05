use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(long)]
    /// Print additional logs and traces
    pub debug: bool,
    #[clap(long)]
    /// Verbose output
    pub verbose: bool,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create and configure a new app from source code or a Docker image
    Launch {
        #[clap(long, default_value = ".")]
        path: String,
    },
}
