//! Based on the <https://github.com/matklad/cargo-xtask> project.
//! This crate is the equivalent of a Makefile, but designed for Rust.

use std::env;

use clap::Parser;
use duct::cmd;

type Result<T> = eyre::Result<T>;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(
    override_usage = "cargo xtask <SUBCOMMAND>",
    subcommand_required = true,
    arg_required_else_help = true,
    trailing_var_arg = true
)]
#[non_exhaustive]
enum Command {
    /// Install cargo requirements
    Setup,
    /// Run any additional checks that are required on CI
    Ci,
    /// Package the software and produce a set of distributable artifacts
    Dist,
    /// Generate the LICENSE file from the licenses of all dependencies
    License,
    /// Generate the table of content in the README.md
    Toc,
}

fn main() -> Result<()> {
    let cargo = env::var("CARGO")
        .ok()
        .and_then(|var| if var.is_empty() { None } else { Some(var) })
        .unwrap_or_else(|| "cargo".to_string());
    let workspace_root = std::env::var("CARGO_WORKSPACE_DIR").unwrap();
    color_eyre::install().unwrap();

    match Command::parse() {
        Command::Setup => {
            let install_if_not_found = |name, features: Option<&str>| {
                cmd!(
                    "sh",
                    "-c",
                    format!(
                        "if ! command -v {name} >/dev/null; then {cargo} install {name} {}; fi",
                        features.map_or_else(
                            || "".to_string(),
                            |features| format!("--features={features}")
                        ),
                    )
                )
            };
            install_if_not_found("cargo-nextest", None).run()?;
            install_if_not_found("cargo-audit", Some("fix")).run()?;
            install_if_not_found("cargo-about", None).run()?;
            install_if_not_found("cargo-deny", None).run()?;
        }
        Command::License => {
            cmd!(
                "sh",
                "-c",
                format!("{cargo} about generate --config {workspace_root}/.cargo/about.toml --workspace {workspace_root}/.cargo/about.hbs -o LICENSE")
            )
            .run()?;
            cmd!(
                "sh",
                "-c",
                [
                    format!("sed -i 's/&quot;/\"/g' {workspace_root}/LICENSE;"),
                    format!("sed -i \"s/&#x27;/'/g\" {workspace_root}/LICENSE;"),
                    format!("sed -i 's/&lt;/</g' {workspace_root}/LICENSE;"),
                    format!("sed -i 's/&gt;/>/g' {workspace_root}/LICENSE;"),
                ]
                .join(" ")
            )
            .run()?;
        }
        Command::Ci => {
            cmd!("sh", "-c", format!("{cargo} nextest run")).run()?;
            cmd!(
                "sh",
                "-c",
                format!("{cargo} deny check --config {workspace_root}/.cargo/deny.toml license")
            )
            .run()?;
            cmd!(
                "sh",
                "-c",
                format!("{cargo} deny check --config {workspace_root}/.cargo/deny.toml bans")
            )
            .run()?;
            cmd!("sh", "-c", format!("{cargo} audit")).run()?;
        }
        Command::Dist => {
            cmd!(
                "sh",
                "-c",
                format!("{cargo} build --release --locked --target=x86_64-unknown-linux-musl")
            )
            .run()?;
            cmd!(
                "sh",
                "-c",
                format!("{cargo} doc --release --locked --target=x86_64-unknown-linux-musl")
            )
            .run()?;
            cmd!("sh", "-c", format!("rm -rf {workspace_root}/dist")).run()?;
            cmd!("sh", "-c", format!("mkdir -p {workspace_root}/dist")).run()?;
            cmd!(
                "sh",
                "-c",
                format!("cp {workspace_root}/LICENSE {workspace_root}/dist")
            )
            .run()?;
            cmd!("sh", "-c", format!("cp {workspace_root}/target/x86_64-unknown-linux-musl/release/sis-server {workspace_root}/dist")).run()?;
            cmd!("sh", "-c", format!("cp {workspace_root}/target/x86_64-unknown-linux-musl/doc {workspace_root}/dist")).run()?;
        }
        Command::Toc => {
            cmd!(
                "sh",
                "-c",
                format!("{workspace_root}/.cargo/gh-md-toc --insert {workspace_root}/README.md")
            )
            .run()?;
            cmd!(
                "sh",
                "-c",
                format!("rm {workspace_root}/README.md.orig.* {workspace_root}/README.md.toc.*")
            )
            .run()?;
        }
    };

    Ok(())
}
