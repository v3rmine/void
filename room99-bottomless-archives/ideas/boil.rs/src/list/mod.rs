use clap::ArgMatches;

use crate::errors::StandardResult;
use crate::utils::config::ConfigIO;
use crate::INSTALL_DIR;

pub fn list(_args: &ArgMatches) -> StandardResult<()> {
    let io = ConfigIO::new()?;

    for template in io.iter() {
        println!("{} {}/{}", template.name, INSTALL_DIR, template.path);
    }

    Ok(())
}
