#![allow(dead_code)]

use std::{
    collections::HashMap, fs::read_to_string, path::PathBuf, str::FromStr
};

use clap::{Parser, Subcommand, ValueHint};
use confique::Config;
use fancy_regex::Captures;
use ouroboros::self_referencing;
use tracing::{debug, info, trace, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use walkdir::{DirEntry, WalkDir};

#[derive(Config, Debug)]
#[config(layer_attr(derive(clap::Args, Debug)))]
struct CliConfig {
    /// Log level
    #[config(
        default = "info",
        layer_attr(arg(short, long, env))
    )]
    log_level: String,

    /// Filetype of the source files
    #[config(default = "md", layer_attr(arg(long, env)))]
    filetype: String,

    /// Params name field identifier
    /// Used by named code blocks
    #[config(
        default = "name",
        layer_attr(arg(long, env))
    )]
    params_name_key: String,

    /// Params file field identifier
    /// Used by file code blocks
    #[config(
        default = "file",
        layer_attr(arg(long, env))
    )]
    params_file_key: String,

    /// Regex to extract code blocks
    #[config(
        default = r#"(?m)(?:(?<=<!--\O*-->\O*)|(?<!<!--\O*))(?<backquotes>^````*+)(?<meta>.*)\n(?<content>\O*?)\n?\k<backquotes>"#,
        layer_attr(arg(long, env))
    )]
    regex_code_block: String,

    /// Regex to extract meta from code blocks
    #[config(
        default = r#"{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}"#,
        layer_attr(arg(long, env))
    )]
    regex_code_meta: String,

    /// Regex to extract parms in meta
    #[config(
        default = r#"(?<key>[[:alnum:]]+)=(?<value>[^\s]+)"#,
        layer_attr(arg(long, env))
    )]
    regex_meta_params: String,

    /// Regex to extract refs in code
    #[config(
        default = r#"(?m)^(?:(?![\t ]+<<)(?<line_indent>[\t ]*+)(?:[^\n<]*+|<)+|(?<direct_indent>[\t ]*))<<(?<ref>[^>]+)>>"#,
        layer_attr(arg(long, env))
    )]
    regex_code_refs: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Use one or multiple configs files
    #[arg(short = 'c', long = "config", value_hint = ValueHint::FilePath, env)]
    config_files: Vec<PathBuf>,

    /// Source dir
    #[arg(short, long, value_hint = ValueHint::DirPath, env)]
    source: String,

    #[command(flatten)]
    cli_config: <CliConfig as Config>::Layer,

    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn build() -> anyhow::Result<(String, CliConfig)> {
        let cli = Cli::parse();
        let mut config =
            CliConfig::builder().preloaded(cli.cli_config);
        for file in cli.config_files {
            config = config.file(file);
        }

        Ok((cli.source, config.load()?))
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check,
}

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
struct IlliterateSourceFile {
    file: String,
    content: String,
    #[borrows(content)]
    #[not_covariant]
    code_blocks: Vec<IlliterateBlock<'this>>,
}

#[derive(Debug)]
enum IlliterateBlock<'a> {
    Named {
        lang: RegexMatchWithString<'a>,
        name: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    },
    File {
        lang: RegexMatchWithString<'a>,
        path: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    },
    Plain {
        lang: RegexMatchWithString<'a>,
        refs_in_code: Vec<IlliterateCodeRef<'a>>,
        params: HashMap<&'a str, RegexMatchWithString<'a>>,
        code_content: RegexMatchWithString<'a>,
        code_block: Captures<'a>,
    }
}

#[derive(Debug)]
struct IlliterateCodeRef<'a> {
    base_indent_match: &'a str,
    is_inline: bool,
    regex_match: RegexMatchWithString<'a>,
}

#[derive(Debug, Copy, Clone)]
struct RegexMatchWithString<'a> {
    content: &'a str,
    regex_match: fancy_regex::Match<'a>,
    content_start_offset: usize,
    content_end_offset: usize,
}

fn main() -> anyhow::Result<()> {
    let (source_dir, cli) = Cli::build()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_str(&cli.log_level)?)
        .init();
    trace!(source = source_dir, config =? &cli, "started app");

    let code_block_regex =
       fancy_regex::Regex::new(&cli.regex_code_block)?;
    let meta_regex =
        fancy_regex::Regex::new(&cli.regex_code_meta)?;
    let param_regex =
        fancy_regex::Regex::new(&cli.regex_meta_params)?;
    let ref_regex =
        fancy_regex::Regex::new(&cli.regex_code_refs)?;

    let walker = WalkDir::new(source_dir).into_iter();
    let _all_files = walker
        .filter_entry(|e| {
            is_dir(e) || is_filetype(e, &cli.filetype)
        })
        .filter_map(|e| e.ok())
        .filter(|e| is_filetype(e, &cli.filetype))
        .map(|entry| -> anyhow::Result<_> {
            let file = entry.path().display().to_string();
            let content = read_to_string(&file)?;
            info!(file, "processing file");
            let source_file: IlliterateSourceFile = IlliterateSourceFileTryBuilder {
                file,
                content,
                code_blocks_builder: |content| {
                    get_code_blocks(content, &cli, &code_block_regex, &meta_regex, &param_regex, &ref_regex)
                },
            }.try_build()?;

            source_file.with(|source_file| {
                debug!(file = source_file.file, "found {} code blocks", source_file.code_blocks.len());
            });

            Ok(source_file)
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(())
}

#[tracing::instrument(level = "trace")]
fn regex_match_from_capture<'a>(
    capture: &fancy_regex::Captures<'a>,
    capture_field_name: &str,
    capture_offset: usize,
) -> anyhow::Result<RegexMatchWithString<'a>> {
    trace!("trying to find a field '{capture_field_name}' in capture");
    capture.name(capture_field_name).map(|m| RegexMatchWithString {
        content: m.as_str(),
        content_start_offset: capture_offset + m.start(),
        content_end_offset: capture_offset + m.end(),
        regex_match: m,
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
    trace!("trying to find '{lang_field_name}' (lang) and '{params_field_name}' (params) in code_match");
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
) -> Vec<IlliterateCodeRef<'a>> {
    trace!("trying to find refs in code block's content");
    // We make a copy outside the closure
    // so the borrow on the match is released at the end of the function
    let start_offset =
        code_match.content_start_offset;

    ref_regex
        .captures_iter(code_match.content)
        .filter_map(Result::ok)
        .map(move |cap| -> anyhow::Result<_> {
            let direct_indent_match = regex_match_from_capture(
                &cap,
                direct_indent_field_name,
                start_offset,
            );
            let line_indent_match = regex_match_from_capture(
                &cap,
                line_indent_field_name,
                start_offset,
            );
            let is_inline = line_indent_match.is_ok();
            let indent = direct_indent_match.unwrap_or_else(|_| line_indent_match.unwrap());

            let ref_match = regex_match_from_capture(
                &cap,
                ref_field_name,
                start_offset,
            )?;

            Ok(IlliterateCodeRef {
                base_indent_match: indent.content,
                regex_match: ref_match,
                is_inline,
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
    let start_offset =
        meta_match.content_start_offset;

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

#[tracing::instrument(level = "trace", skip(content, config))]
fn get_code_blocks<'a>(
    content: &'a str,
    config: &CliConfig,
    code_block_regex: &fancy_regex::Regex,
    meta_regex: &fancy_regex::Regex,
    param_regex: &fancy_regex::Regex,
    ref_regex: &fancy_regex::Regex,
) -> anyhow::Result<Vec<IlliterateBlock<'a>>> {
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
                    &meta_regex,
                    "lang",
                    "params",
                )?;

            Ok((
                code_block,
                code_content,
                meta_lang,
                meta_params,
                refs_from_code_match(&code_content, ref_regex, "direct_indent", "line_indent", "ref"),
            ))
        })
        .filter_map(Result::ok)
        .map(|(code_block, code_content, lang, meta_params, refs_in_code)| {
            let params = params_from_meta_match(&meta_params, param_regex, "key", "value");

            if let Some(file_value) = params.get(config.params_file_key.as_str()) {
                return IlliterateBlock::File {
                    path: *file_value,
                    lang,
                    code_content,
                    code_block,
                    params,
                    refs_in_code,
                }
            }
            if let Some(ref_value) = params.get(config.params_name_key.as_str()) {
                return IlliterateBlock::Named {
                    name: *ref_value,
                    lang,
                    code_content,
                    code_block,
                    params,
                    refs_in_code,
                }
            }
            IlliterateBlock::Plain {
                lang,
                code_content,
                code_block,
                params,
                refs_in_code,
            }
        });

    Ok(code_blocks.collect::<Vec<_>>())
}
