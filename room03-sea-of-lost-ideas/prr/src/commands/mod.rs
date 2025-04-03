use crate::{cli::Commands, EyreResult};
use clap::StructOpt;

mod define_package;

pub fn handle_commands() -> EyreResult<()> {
    match Commands::parse() {
        Commands::DefinePackage(args) => define_package::handle_define_package(args)?,
        _ => unimplemented!(),
    }

    Ok(())
}
