use std::process::ExitCode;
use std::time::Duration;

use notify_debouncer_mini::{
    DebounceEventResult, new_debouncer,
};
use tracing::{error, info, warn};

use crate::cli::CliConfig;

pub fn watch(
    source_dir: &str,
    cli: &CliConfig,
    outdir: &str,
    poll: bool,
) -> ExitCode {
    info!(
        source = source_dir,
        outdir, poll, "starting watch mode"
    );

    let _code = crate::commands::tangle::tangle(
        source_dir, cli, outdir, false,
    );
    info!("initial tangle complete");

    let cli_clone = cli.clone();
    let source_dir_clone = source_dir.to_string();
    let outdir_clone = outdir.to_string();

    let mut debouncer = match new_debouncer(
        Duration::from_millis(300),
        move |result: DebounceEventResult| match result {
            Ok(events) => {
                for event in &events {
                    let is_relevant = event
                        .path
                        .extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e == cli_clone.filetype)
                        .unwrap_or(false);
                    if is_relevant {
                        let paths: Vec<_> = events
                            .iter()
                            .map(|e| &e.path)
                            .collect();
                        info!(
                            paths = ?paths,
                            "file change detected, re-tangling"
                        );
                        let _ =
                            crate::commands::tangle::tangle(
                                &source_dir_clone,
                                &cli_clone,
                                &outdir_clone,
                                false,
                            );
                    }
                }
            }
            Err(collect_error) => {
                warn!(error = ?collect_error, "watch error");
            }
        },
    ) {
        Ok(d) => d,
        Err(e) => {
            error!(error = %e, "failed to create file watcher");
            return ExitCode::FAILURE;
        }
    };

    if poll {
        info!("using polling watcher");
    }

    let source_path = std::path::Path::new(source_dir);
    if let Err(e) = debouncer.watcher().watch(
        source_path,
        notify_debouncer_mini::notify::RecursiveMode::Recursive,
    ) {
        error!(error = %e, "failed to watch source directory");
        return ExitCode::FAILURE;
    }

    info!("watching for changes, press Ctrl+C to stop");

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
