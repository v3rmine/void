use std::collections::HashMap;

use tracing::trace;
use types::{
    IlliterateCodeWithRefs, IlliterateResolvedResult,
};

pub mod cli;
pub mod commands;
pub mod parse;
pub mod types;

/// Based on https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
pub fn resolve_references(
    blocks: HashMap<String, IlliterateCodeWithRefs>,
) -> IlliterateResolvedResult {
    let mut resolved: HashMap<String, String> =
        HashMap::new();
    let mut refs_deps_to_resolve_count: HashMap<
        String,
        usize,
    > = HashMap::new();
    let mut dependants: HashMap<String, Vec<String>> =
        HashMap::new();

    for (name, data) in &blocks {
        refs_deps_to_resolve_count
            .entry(name.clone())
            .or_insert(0);
        for dep in &data.refs_in_code {
            if blocks.contains_key(&dep.name) {
                dependants
                    .entry(dep.name.clone())
                    .or_default()
                    .push(name.clone());
                *refs_deps_to_resolve_count
                    .entry(name.clone())
                    .or_insert(0) += 1;
            }
        }
    }

    let mut refs_without_deps: Vec<String> =
        refs_deps_to_resolve_count
            .iter()
            .filter(|&(_, count)| *count == 0)
            .map(|(name, _)| name.clone())
            .collect();

    while let Some(name) = refs_without_deps.pop() {
        let block = blocks.get(&name).unwrap();
        let mut content = block.code_content.clone();

        for dep_ref in &block.refs_in_code {
            if let Some(dep_content) =
                resolved.get(&dep_ref.name)
            {
                // Handle indentation for the reference replacement
                let replacement = if dep_ref.is_inline {
                    // For inline references, no indentation adjustment needed
                    dep_content.clone()
                } else {
                    // For block references, apply the reference's indentation
                    let indent = &dep_ref.base_indent_match;
                    dep_content
                        .lines()
                        .enumerate()
                        .map(|(i, line)| {
                            if i == 0 || line.is_empty() {
                                // First line already have indentation
                                // And empty lines get no additional indentation
                                line.to_string()
                            } else {
                                // Subsequent lines get the reference's indentation
                                format!(
                                    "{}{}",
                                    indent, line
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                content = content.replace(
                    &dep_ref.ref_text,
                    &replacement,
                );
            }
        }

        resolved.insert(name.clone(), content);

        if let Some(deps) = dependants.get(&name) {
            for dependent in deps {
                if let Some(count) =
                    refs_deps_to_resolve_count
                        .get_mut(dependent)
                {
                    *count -= 1;
                    if *count == 0 {
                        refs_without_deps
                            .push(dependent.clone());
                    }
                }
            }
        }
    }

    let cyclic: Vec<_> = blocks
        .keys()
        .filter(|name| !resolved.contains_key(*name))
        .cloned()
        .collect();

    let all_known: Vec<_> = blocks.keys().collect();
    let mut missing = Vec::new();
    for (name, data) in &blocks {
        for dep in &data.refs_in_code {
            if !all_known.contains(&&dep.name) {
                missing
                    .push((name.clone(), dep.name.clone()));
            }
        }
    }

    IlliterateResolvedResult {
        resolved,
        cyclic,
        missing,
    }
}

pub fn collect_code_blocks(
    files: Vec<types::IlliterateSourceFile>,
) -> HashMap<String, IlliterateCodeWithRefs> {
    let mut blocks: HashMap<
        String,
        IlliterateCodeWithRefs,
    > = HashMap::new();

    for file in &files {
        for block in &file.code_blocks {
            match block {
                types::IlliterateBlock::Named {
                    name,
                    code_content,
                    refs_in_code,
                    ..
                } => {
                    if let Some(existing) =
                        blocks.get_mut(name)
                    {
                        trace!(
                            "duplicate named block '{name}', joining with newline"
                        );
                        existing.code_content.push('\n');
                        existing
                            .code_content
                            .push_str(code_content);
                        existing
                            .refs_in_code
                            .extend(refs_in_code.to_vec());
                    } else {
                        blocks.insert(
                            name.clone(),
                            IlliterateCodeWithRefs {
                                code_content: code_content
                                    .clone(),
                                refs_in_code: refs_in_code
                                    .clone(),
                            },
                        );
                    }
                }
                types::IlliterateBlock::File {
                    path,
                    code_content,
                    refs_in_code,
                    ..
                } => {
                    blocks.insert(
                        format!("__file__:{path}"),
                        IlliterateCodeWithRefs {
                            code_content: code_content
                                .clone(),
                            refs_in_code: refs_in_code
                                .clone(),
                        },
                    );
                }
                _ => {}
            }
        }
    }

    blocks
}
