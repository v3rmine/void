use std::{fs, path::Path, process::ExitCode};

use tracing::{debug, error, info, trace, warn};

use crate::{
    cli::CliConfig,
    collect_code_blocks, parse, resolve_references,
    types::{
        IlliterateResolvedResult, IlliterateSourceFile,
    },
};

pub fn tangle(
    source_dir: &str,
    cli: &CliConfig,
    outdir: &str,
    dry_run: bool,
) -> ExitCode {
    let files = match parse::crawl_source_dir(
        source_dir, cli,
    ) {
        Ok(f) => f,
        Err(e) => {
            error!(error = %e, "failed to crawl source directory");
            return ExitCode::FAILURE;
        }
    };

    let files = files
        .into_iter()
        .map(IlliterateSourceFile::from)
        .collect::<Vec<_>>();

    let blocks = collect_code_blocks(files);
    let resolved = resolve_references(blocks);
    tangle_result(&resolved, outdir, dry_run)
}

fn tangle_result(
    result: &IlliterateResolvedResult,
    outdir: &str,
    dry_run: bool,
) -> ExitCode {
    for (block, missing_ref) in &result.missing {
        warn!(
            "block '{block}' references missing block '{missing_ref}'"
        );
    }

    if !result.cyclic.is_empty() {
        warn!(
            "circular dependency detected in blocks: {}",
            result.cyclic.join(", ")
        );
    }

    for (name, content) in &result.resolved {
        if name.starts_with("__file__:") {
            let path =
                name.strip_prefix("__file__:").unwrap();
            debug!(
                path,
                len = content.len(),
                "resolved file block"
            );
        } else {
            trace!(
                name,
                len = content.len(),
                "resolved named block"
            );
        }
    }

    if dry_run {
        for (name, content) in &result.resolved {
            if let Some(path) =
                name.strip_prefix("__file__:")
            {
                debug!(path, content, "would emit file");
            }
        }
    } else {
        let outdir = Path::new(outdir);
        fs::create_dir_all(outdir).unwrap();

        for (name, content) in &result.resolved {
            if let Some(path) =
                name.strip_prefix("__file__:")
            {
                let out_path = outdir.join(path);
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::write(&out_path, content).unwrap();
                info!(path, "emitted file");
            }
        }
    }

    if !result.missing.is_empty() {
        return ExitCode::from(2);
    }

    ExitCode::SUCCESS
}
