use std::process::ExitStatus;

use clap::Parser;

type Result<T> = eyre::Result<T>;

mod generate_docker_models;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(
    name = "docker-cd-toolbox",
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
    /// Generate Docker API models
    GenDockerModels,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    color_eyre::install().unwrap();

    match Command::parse() {
        Command::Nextest => exec("cargo nextest run --package parser").await.unwrap(),
        Command::Test => exec("cargo test --package parser").await.unwrap(),
        Command::DbStatus => exec("cargo run --package migration -- status")
            .await
            .unwrap(),
        Command::DbApply => exec("cargo run --package migration -- up").await.unwrap(),
        Command::GenDockerModels => generate_docker_models::generate_docker_models()
            .await
            .unwrap(),
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
