use clap::Parser;

mod apps;
mod auth;
mod config;
mod machine;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
/// This is dedalectl, the Fly.. **cough** poor man command line interface to reach the sun.
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    #[clap(subcommand, about = "Manage authentication")]
    Auth(auth::Command),
    #[clap(subcommand, about = "Manage apps")]
    Apps(apps::Command),
    #[clap(subcommand, about = "Manage machines")]
    Machine(machine::Command),
}

fn main() {
    let _config = config::Config::parse();
    let _app = App::parse();
}
