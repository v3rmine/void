use std::process::ExitStatus;

use clap::Parser;

type Result<T> = eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(
    name = "arsonist-toolbox",
    override_usage = "cargo xtask <SUBCOMMAND>",
    subcommand_required = true,
    arg_required_else_help = true
)]
enum Command {
    /// Run the tests using https://nexte.st/
    Nextest,
    /// Run the tests using cargo test
    Test,
    /// Show the migrations status in the database
    DbStatus,
    /// Apply the pending migrations to the database,
    DbApply,
    /// Create a new service,
    NewService {
        /// The name of the service
        name: String,
    },
    /// Create a new app,
    NewApp {
        /// The name of the app
        name: String,
    },
    /// Regenerate the models from the database
    GenerateModels,
}

#[tokio::main]
async fn main() {
    let workspace_root = std::env::var("CARGO_WORKSPACE_DIR").unwrap();
    color_eyre::install().unwrap();

    match Command::parse() {
        Command::Nextest => exec("cargo nextest run --package parser").await.unwrap(),
        Command::Test => exec("cargo test --package parser").await.unwrap(),
        Command::DbStatus => exec("cargo run --package migration -- status")
            .await
            .unwrap(),
        Command::DbApply => exec("cargo run --package migration -- up").await.unwrap(),
        Command::NewService { name } => exec(&format!(
            "cargo new --lib --name {name} {workspace_root}services/{name}",
        ))
        .await
        .unwrap(),
        Command::NewApp { name } => exec(&format!(
            "cargo new --bin --name {name} {workspace_root}apps/{name}"
        ))
        .await
        .unwrap(),
        Command::GenerateModels => {
            exec("sea-orm-cli generate entity --output-dir {workspace_root}services/models/src/entities")
                .await
                .unwrap();
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ExecOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

async fn _exec_catch(command: &str) -> Result<ExecOutput> {
    let command = command.split_ascii_whitespace().collect::<Vec<&str>>();

    let output = tokio::process::Command::new(command[0])
        .args(&command[1..])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(ExecOutput {
        status: output.status,
        stdout,
        stderr,
    })
}

#[allow(dead_code)]
async fn exec(command: &str) -> Result<()> {
    let splitted_command = command.split_ascii_whitespace().collect::<Vec<&str>>();

    let output = tokio::process::Command::new(splitted_command[0])
        .args(&splitted_command[1..])
        .spawn()?
        .wait_with_output()
        .await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(eyre::eyre!(
            "Error, command {} exited with status code {}",
            command,
            output
                .status
                .code()
                .map_or("KILLED BY SIGNAL".to_string(), |c| c.to_string())
        ))
    }
}

#[allow(dead_code)]
async fn exec_script(command: &str) -> eyre::Result<()> {
    let output = tokio::process::Command::new("bash")
        .args(&["-c", command])
        .spawn()?
        .wait_with_output()
        .await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(eyre::eyre!(
            "Error, command {} exited with status code {}",
            command,
            output
                .status
                .code()
                .map_or("KILLED BY SIGNAL".to_string(), |c| c.to_string())
        ))
    }
}
