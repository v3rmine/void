use crate::{
    cli::{setup_logging, DefinePackageArgs},
    packages_definitions::PackagesDefinitions,
    EyreResult,
};

pub fn handle_define_package(args: DefinePackageArgs) -> EyreResult<()> {
    setup_logging(args.verbose)?;

    let packages = PackagesDefinitions::get_from_stdio();
    println!("{}", packages.to_json());

    Ok(())
}
