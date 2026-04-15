use clap::Parser;

mod cli;
mod client;
mod server;
use cli::*;

pub type EyreResult<Output> = color_eyre::eyre::Result<Output>;

pub const HELLO_MSG: &[u8] = b"PING";
pub const EMPTY_MSG: &[u8] = b"";
pub const ACK_MSG: &[u8] = b"PONG";
pub const MSG_BUFFER_LENGTH: usize = 16;

fn setup_logging(verbosity: usize) -> EyreResult<()> {
    if verbosity > 0 {
        color_eyre::install().unwrap();
    }

    simple_logger::SimpleLogger::new()
        .env()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(match verbosity {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Info,
            _ => log::LevelFilter::Debug,
        })
        .with_module_level("trust_dns_proto", log::LevelFilter::Error)
        .with_module_level("trust_dns_resolver", log::LevelFilter::Error)
        .init()?;

    Ok(())
}

fn main() -> EyreResult<()> {
    let runtime = tokio::runtime::Runtime::new()?;

    match Commands::parse() {
        Commands::Client(args) => {
            runtime.block_on(async { client::handle_client(args).await })?;
        }
        Commands::Server(args) => {
            runtime.block_on(async { server::handle_server(args).await })?;
        }
    }

    Ok(())
}
