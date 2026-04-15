use std::process::ExitCode;

use rayon::prelude::*;
use tracing::{error, info, trace, warn};

use crate::{
    cli::CliConfig,
    collect_code_blocks, parse, resolve_references,
    types::{
        IlliterateResolvedResult, IlliterateSourceFile,
    },
};

pub fn check(
    source_dir: &str,
    cli: &CliConfig,
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
        .into_par_iter()
        .map(IlliterateSourceFile::from)
        .collect::<Vec<_>>();

    let blocks = collect_code_blocks(files);
    let resolved = resolve_references(blocks);
    check_result(&resolved)
}

fn check_result(
    result: &IlliterateResolvedResult,
) -> ExitCode {
    for (block, missing_ref) in &result.missing {
        warn!(
            "block '{block}' references missing block '{missing_ref}'"
        );
    }

    if !result.cyclic.is_empty() {
        error!(
            "circular dependency detected in blocks: {}",
            result.cyclic.join(", ")
        );
        return ExitCode::from(1);
    }

    if !result.missing.is_empty() {
        return ExitCode::from(2);
    }

    for (name, content) in &result.resolved {
        if name.starts_with("__file__:") {
            let path =
                name.strip_prefix("__file__:").unwrap();
            info!(
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

    ExitCode::SUCCESS
}
