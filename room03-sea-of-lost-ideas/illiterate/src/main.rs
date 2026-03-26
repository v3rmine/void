#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr};

use tracing::{error, trace};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use types::{
    IlliterateBlock, IlliterateRef, IlliterateSourceFile,
};

mod cli;
mod parse;
mod types;

fn main() -> anyhow::Result<()> {
    let (source_dir, cli) = cli::Cli::build()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_str(&cli.log_level)?)
        .init();
    trace!(source = source_dir, config =? &cli, "started app");

    let files = parse::crawl_source_dir(&source_dir, &cli)?
        .into_iter()
        .map(IlliterateSourceFile::from)
        .collect::<Vec<_>>();

    let mut refs_to_code: HashMap<String, Vec<String>> =
        HashMap::new();
    // let mut files_to_output: HashMap<&str, &str> =
    //     HashMap::new();

    let mut refs_to_resolve: HashMap<
        String,
        (Vec<IlliterateRef>, String),
    > = files
        .into_iter()
        .flat_map(|file| file.code_blocks.into_iter())
        .filter_map(|block| match block {
            IlliterateBlock::Named {
                name,
                code_content,
                refs_in_code,
                ..
            } => {
                if refs_in_code.len() > 0 {
                    Some((
                        name,
                        (
                            refs_in_code,
                            code_content,
                        ),
                    ))
                } else {
                    if let Some(codes) =
                        refs_to_code.get_mut(&name)
                    {
                        codes.push(code_content);
                    } else {
                        refs_to_code.insert(
                            name,
                            vec![code_content],
                        );
                    }
                    None
                }
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut refs_to_check =
        refs_to_resolve.clone().into_iter();

    while let Some((name, (dependencies, code))) =
        refs_to_check.next()
    {
        let mut valid_deps_to_replace = Vec::new();
        let mut unresolved_deps = Vec::new();
        let missing_deps = dependencies
            .into_iter()
            .filter_map(|dep| {
                if let Some(plain_code_ref) =
                    refs_to_code.get(&dep.name)
                {
                    valid_deps_to_replace.push((
                        dep.ref_text,
                        plain_code_ref,
                    ));
                    return None;
                }

                if refs_to_resolve.contains_key(&dep.name) {
                    unresolved_deps.push(dep);
                    None
                } else {
                    Some(dep.name)
                }
            })
            .collect::<Vec<_>>();
        if !missing_deps.is_empty() {
            refs_to_resolve.remove(&name);
            error!(
                "named block {name} depend on missing refs: {}",
                missing_deps.join(", ")
            )
        } else {
            let mut new_content = code.to_string();
            for (anchor, contents) in valid_deps_to_replace.into_iter() {
                // TODO: Work the replacement
                new_content = new_content.replace(&anchor, &contents.join("\n"));
            }

            if unresolved_deps.len() == 0 {
                refs_to_resolve.remove(&name);
                if let Some(codes) =
                    refs_to_code.get_mut(&name)
                {
                    codes.push(new_content);
                } else {
                    refs_to_code.insert(
                        name,
                        vec![new_content],
                    );
                }
            } else {
                refs_to_check = refs_to_resolve.clone().into_iter();
                // if let Some(original_ref_to_resolve) = refs_to_resolve.get_mut(name) {
                //     original_ref_to_resolve
                // }
            }
        }
    }

    dbg!(refs_to_code);
    dbg!(refs_to_resolve);
    // dbg!(files_to_output);

    Ok(())
}
