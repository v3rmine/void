use std::process::ExitStatus;

use clap::{CommandFactory, Parser};

type IoResult<T> = Result<T, std::io::Error>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(
    name = "org-empire-toolbox",
    override_usage = "cargo xtask <SUBCOMMAND>",
    subcommand_required = true,
    arg_required_else_help = true
)]
enum Command {
    NextestParser,
    TestParser,
    Install,
    Uninstall,
    Lint,
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ExecOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

async fn _exec_catch(command: &str) -> IoResult<ExecOutput> {
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

async fn exec(command: &str) -> IoResult<()> {
    let command = command.split_ascii_whitespace().collect::<Vec<&str>>();

    tokio::process::Command::new(command[0])
        .args(&command[1..])
        .spawn()?
        .wait_with_output()
        .await?;

    Ok(())
}

async fn async_main() {
    match Command::parse() {
        Command::NextestParser => exec("cargo nextest run --package org-parser")
            .await
            .unwrap(),
        Command::TestParser => exec("cargo test --package org-parser").await.unwrap(),
        _ => {
            <Command as CommandFactory>::command().print_help().unwrap();
        }
    }
}
