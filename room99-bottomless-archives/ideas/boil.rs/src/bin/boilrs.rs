#![recursion_limit = "1024"]

use anyhow::{Context, Result};
use human_panic::setup_panic;

use boilrs::{generate, install, list, new, uninstall, utils};

fn main() -> Result<()> {
    setup_panic!();

    // * Creating cli app
    let cli = utils::terminal::init_term().get_matches();

    match cli.subcommand() {
        ("generate", Some(args)) => generate::generate(&args).context(format!(
            "Failed to generate the template using `args`: {:?}",
            args
        ))?,
        ("new", Some(args)) => new::new(&args).context(format!(
            "Failed to create new config using `args`: {:?}",
            args
        ))?,
        ("install", Some(args)) => install::install(args).context(format!(
            "Failed to install the template using `args`: {:?}",
            args
        ))?,
        ("uninstall", Some(args)) => uninstall::uninstall(args).context(format!(
            "Failed to uninstall the template using `args`: {:?}",
            args
        ))?,
        ("list", Some(args)) => list::list(args).context(format!(
            "Failed to list the templates using `args`: {:?}",
            args
        ))?,
        // ("download", Some(_args)) => unimplemented!(),
        // ("update", Some(_args)) => unimplemented!(),
        _ => (),
    }

    Ok(())
}
