mod cli;
mod commands;
mod packages_definitions;

pub type EyreResult<Output> = color_eyre::eyre::Result<Output>;

fn main() -> EyreResult<()> {
    commands::handle_commands()?;

    Ok(())
}
