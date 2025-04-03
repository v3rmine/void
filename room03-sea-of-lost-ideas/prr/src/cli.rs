use clap::Parser;

use crate::EyreResult;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub enum Commands {
    IsSupported,
    #[clap(alias = "def-p")]
    DefinePackage(DefinePackageArgs),
    Prompt,
    Correct,
    Install,
}

#[derive(clap::Args, Debug)]
#[clap(author, version, about = "Define package to install (aliases [def-p])")]
pub struct DefinePackageArgs {
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
    #[clap(short = 'O', long)]
    pub os: Option<String>,
    #[clap(short, long)]
    pub optional: bool,
    #[clap(short = 'p', long, alias = "name")]
    pub package_name: Option<String>,
    #[clap(short = 'P', long, alias = "ver")]
    pub package_version: Option<String>,
    #[clap(short, long)]
    pub stdio: bool,
    #[clap(short, long)]
    pub raw: Option<String>,
}

pub fn setup_logging(verbosity: usize) -> EyreResult<()> {
    if verbosity > 0 {
        color_eyre::install().unwrap();
    }

    Ok(())
}
