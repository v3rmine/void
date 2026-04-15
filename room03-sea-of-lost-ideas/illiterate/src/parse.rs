use std::{collections::HashMap, fs::read_to_string};

use fancy_regex::Captures;
use ouroboros::self_referencing;
use tracing::{debug, info, trace, warn};
use walkdir::{DirEntry, WalkDir};

use crate::cli::CliConfig;

fn is_filetype(entry: &DirEntry, filetype: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(&format!(".{filetype}")))
        .unwrap_or(false)
}
fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

#[self_referencing]
pub struct IlliterateParserSourceFile {
    pub file: String,
    pub content: String,
    #[borrows(content)]
    #[not_covariant]
    pub code_blocks: Vec<IlliterateParserBlock<'this>>,
}

#[derive(Debug)]
pub enum IlliterateParserBlock<'a> {
    #[non_exhaustive]
    Named {
        lang: RegexMatchWithString<'a>,
        name: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateParserCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    },
    #[non_exhaustive]
    File {
        lang: RegexMatchWithString<'a>,
        path: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateParserCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    },
    #[non_exhaustive]
    Plain {
        lang: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateParserCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub struct IlliterateParserCodeRef<'a> {
    pub base_indent_match: &'a str,
    pub is_inline: bool,
    pub regex_match: RegexMatchWithString<'a>,
    pub full_ref_match: RegexMatchWithString<'a>,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct RegexMatchWithString<'a> {
    pub content: &'a str,
    _regex_match: fancy_regex::Match<'a>,
    pub content_start_offset: usize,
    pub content_end_offset: usize,
}

#[tracing::instrument(level = "trace")]
fn regex_match_from_capture<'a>(
    capture: &fancy_regex::Captures<'a>,
    capture_field_name: &str,
    capture_offset: usize,
) -> anyhow::Result<RegexMatchWithString<'a>> {
    trace!(
        "trying to find a field '{capture_field_name}' in capture"
    );
    capture.name(capture_field_name).map(|m| RegexMatchWithString {
        content: m.as_str(),
        content_start_offset: capture_offset + m.start(),
        content_end_offset: capture_offset + m.end(),
        _regex_match: m,
    }).ok_or_else(|| {
        debug!(capture_field_name, "no field found on capture");
        anyhow::anyhow!("no {capture_field_name} field found on capture")
    })
}

#[tracing::instrument(level = "trace")]
fn meta_from_code_match<'a>(
    code_match: &RegexMatchWithString<'a>,
    meta_regex: &fancy_regex::Regex,
    lang_field_name: &str,
    params_field_name: &str,
) -> anyhow::Result<(
    RegexMatchWithString<'a>,
    RegexMatchWithString<'a>,
)> {
    trace!(
        "trying to find '{lang_field_name}' (lang) and '{params_field_name}' (params) in code_match"
    );
    meta_regex
        .captures(code_match.content)
        .ok()
        .flatten()
        .map(|cap| -> anyhow::Result<_> {
            Ok((
                regex_match_from_capture(&cap, lang_field_name, code_match.content_start_offset)?,
                regex_match_from_capture(&cap, params_field_name, code_match.content_start_offset)?,
            ))
        })
        .and_then(Result::ok)
        .ok_or_else(|| {
            debug!(lang_field_name, params_field_name, "failed to extract lang and params from code_meta");
            anyhow::anyhow!("failed to extract '{lang_field_name}' (lang) and '{params_field_name}' (params) from code_meta")
        })
}

#[tracing::instrument(level = "trace")]
fn refs_from_code_match<'a>(
    code_match: &RegexMatchWithString<'a>,
    ref_regex: &fancy_regex::Regex,
    direct_indent_field_name: &'a str,
    line_indent_field_name: &'a str,
    ref_field_name: &'a str,
    full_ref_field_name: &'a str,
) -> Vec<IlliterateParserCodeRef<'a>> {
    trace!("trying to find refs in code block's content");
    // We make a copy outside the closure
    // so the borrow on the match is released at the end of the function
    let start_offset = code_match.content_start_offset;

    ref_regex
        .captures_iter(code_match.content)
        .filter_map(Result::ok)
        .map(move |cap| -> anyhow::Result<_> {
            let direct_indent_match =
                regex_match_from_capture(
                    &cap,
                    direct_indent_field_name,
                    start_offset,
                );
            let line_indent_match =
                regex_match_from_capture(
                    &cap,
                    line_indent_field_name,
                    start_offset,
                );
            let is_inline = line_indent_match.is_ok();
            let indent = direct_indent_match
                .unwrap_or_else(|_| {
                    line_indent_match.unwrap()
                });

            let ref_match = regex_match_from_capture(
                &cap,
                ref_field_name,
                start_offset,
            )?;
            let full_ref_match = regex_match_from_capture(
                &cap,
                full_ref_field_name,
                start_offset,
            )?;

            Ok(IlliterateParserCodeRef {
                base_indent_match: indent.content,
                regex_match: ref_match,
                is_inline,
                full_ref_match,
            })
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

#[tracing::instrument(level = "trace")]
fn params_from_meta_match<'a>(
    meta_match: &RegexMatchWithString<'a>,
    param_regex: &fancy_regex::Regex,
    key_field_name: &'a str,
    value_field_name: &'a str,
) -> HashMap<&'a str, RegexMatchWithString<'a>> {
    trace!("trying to find params in code block's meta");
    // We make a copy outside the closure
    // so the borrow on the match is released at the end of the function
    let start_offset = meta_match.content_start_offset;

    param_regex
        .captures_iter(meta_match.content)
        .filter_map(Result::ok)
        .map(move |cap| -> anyhow::Result<_> {
            let key = regex_match_from_capture(
                &cap,
                key_field_name,
                start_offset,
            )?;
            let value = regex_match_from_capture(
                &cap,
                value_field_name,
                start_offset,
            )?;

            Ok((key.content, value))
        })
        .filter_map(Result::ok)
        .collect::<HashMap<_, _>>()
}

#[tracing::instrument(
    level = "trace",
    skip(content, config)
)]
fn get_code_blocks<'a>(
    content: &'a str,
    config: &CliConfig,
    code_block_regex: &fancy_regex::Regex,
    meta_regex: &fancy_regex::Regex,
    param_regex: &fancy_regex::Regex,
    ref_regex: &fancy_regex::Regex,
) -> anyhow::Result<Vec<IlliterateParserBlock<'a>>> {
    trace!("finding code blocks in file");
    let code_blocks = code_block_regex
        .captures_iter(content)
        .filter_map(Result::ok)
        .map(|cap| -> anyhow::Result<_> {
            let meta = regex_match_from_capture(
                &cap, "meta", 0,
            )?;
            let content = regex_match_from_capture(
                &cap, "content", 0,
            )?;

            Ok((
                cap,
                meta,
                content,
            ))
        })
        .filter_map(Result::ok)
        .map(|(code_block, code_meta, code_content)| -> anyhow::Result<_> {
            let (meta_lang, meta_params) =
                meta_from_code_match(
                    &code_meta,
                    meta_regex,
                    "lang",
                    "params",
                )?;

            Ok((
                code_block,
                code_content,
                meta_lang,
                meta_params,
                refs_from_code_match(&code_content, ref_regex, "direct_indent", "line_indent", "ref", "full_ref"),
            ))
        })
        .filter_map(Result::ok)
        .map(|(code_block, code_content, lang, meta_params, refs_in_code)| {
            let params = params_from_meta_match(&meta_params, param_regex, "key", "value");

            if let Some(file_value) = params.get(config.params_file_key.as_str()) {
                return IlliterateParserBlock::File {
                    path: *file_value,
                    lang,
                    code_content,
                    code_block,
                    params,
                    refs_in_code,
                }
            }
            if let Some(ref_value) = params.get(config.params_name_key.as_str()) {
                return IlliterateParserBlock::Named {
                    name: *ref_value,
                    lang,
                    code_content,
                    code_block,
                    params,
                    refs_in_code,
                }
            }
            IlliterateParserBlock::Plain {
                lang,
                code_content,
                code_block,
                params,
                refs_in_code,
            }
        });

    Ok(code_blocks.collect::<Vec<_>>())
}

#[tracing::instrument(level = "trace", skip(config))]
pub fn crawl_source_dir(
    source_dir: &str,
    config: &CliConfig,
) -> anyhow::Result<Vec<IlliterateParserSourceFile>> {
    let code_block_regex =
        fancy_regex::Regex::new(&config.regex_code_block)?;
    let meta_regex =
        fancy_regex::Regex::new(&config.regex_code_meta)?;
    let param_regex =
        fancy_regex::Regex::new(&config.regex_meta_params)?;
    let ref_regex =
        fancy_regex::Regex::new(&config.regex_code_refs)?;

    let walker = WalkDir::new(source_dir).into_iter();
    let all_files = walker
        .filter_entry(|e| {
            is_dir(e) || is_filetype(e, &config.filetype)
        })
        .filter_map(|e| e.ok())
        .filter(|e| is_filetype(e, &config.filetype))
        .map(|entry| -> anyhow::Result<_> {
            let file = entry.path().display().to_string();
            let content = read_to_string(&file)?;
            info!(file, "processing file");
            let source_file: IlliterateParserSourceFile =
                IlliterateParserSourceFileTryBuilder {
                    file,
                    content,
                    code_blocks_builder: |content| {
                        get_code_blocks(
                            content,
                            config,
                            &code_block_regex,
                            &meta_regex,
                            &param_regex,
                            &ref_regex,
                        )
                    },
                }
                .try_build()?;

            source_file.with(|source_file| {
                debug!(
                    file = source_file.file,
                    "found {} code blocks",
                    source_file.code_blocks.len()
                );
            });

            Ok(source_file)
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(all_files)
}
