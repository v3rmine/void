use std::{env::set_var, fs::create_dir_all, path::Path};

use clap::Parser;
use eyre::Result;
use fuser::MountOption;
use log_utils::info;

mod filesystem;
mod project;
mod project_env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true, arg_required_else_help = true)]
struct Cli {
    #[clap(long, env, visible_alias = "host", default_value = "gitlab.com")]
    gitlab_host: String,
    #[clap(long, env, visible_alias = "access-token")]
    gitlab_access_token: String,
    #[clap(long, env, default_value = "./mnt")]
    mountpoint: String,
    /// -v for info ; -vv for debug ; -vvv for trace (shortcut to set LOG_LEVEL)
    #[clap(short, long = "verbose", action = clap::ArgAction::Count)]
    verbosity: u8,
    /// The Gitlab query to filter projects
    #[clap(default_value = "")]
    query: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = Cli::parse();

    if app.verbosity > 2 {
        set_var("LOG_LEVEL", "trace");
    } else if app.verbosity == 2 {
        set_var("LOG_LEVEL", "debug");
    } else if app.verbosity == 1 {
        set_var("LOG_LEVEL", "info");
    }

    // Setup the logger (tracing)
    log_utils::setup_simple_logger();

    let fs = filesystem::GitlabFS::new(app.gitlab_host, app.gitlab_access_token, app.query);

    // Mount the filesystem with a custom name "gitlabfs" as Read and Write
    let fuse_args = [MountOption::FSName("gitlabfs".to_string()), MountOption::RW];

    // Create the mountpoint if it does not exist
    let mountpoint = Path::new(&app.mountpoint);
    if !mountpoint.exists() {
        create_dir_all(mountpoint).unwrap();
    }
    // Spawn the filesystem in the background
    let _filesystem_handle: fuser::BackgroundSession =
        fuser::spawn_mount2(fs, mountpoint, &fuse_args).unwrap();

    // Handle CTRL-C to close the filesystem gracefully
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        info!("Termination signal catched, closing the filesystem gracefully");
        tx.send(())
            .expect("Failed to forward the termination signal to the main thread");
    })
    .expect("Error setting Ctrl-C handler");

    // Do not exit until CTRL-C is pressed
    rx.recv().expect("Could not wait for gracefull shutdown");

    // REVIEW: May not be needed
    // filesystem_handle.join();
    // info!("Filesystem closed gracefully");

    Ok(())
}
