use std::{
    collections::HashMap, fs::read_to_string,
    str::FromStr,
};

use clap::{
    CommandFactory, FromArgMatches, Parser, Subcommand,
    ValueHint, parser::ValueSource,
};
use config::Config;
use serde::Deserialize;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug, Deserialize)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_hint = ValueHint::FilePath, env)]
    config: Option<String>,

    /// Log level
    #[arg(short, long, default_value_t = default_log_level(), env)]
    #[serde(default = "default_log_level")]
    log_level: String,

    /// Source dir
    #[arg(short, long, value_hint = ValueHint::DirPath, env)]
    source: String,

    /// Filetype of the source files
    #[arg(long, default_value_t = default_filetype(), env)]
    #[serde(default = "default_filetype")]
    filetype: String,

    /// Regex to extract code blocks
    #[arg(long, default_value_t = default_regex_code_block(), env)]
    #[serde(default = "default_regex_code_block")]
    regex_code_block: String,

    /// Regex to extract meta from code blocks
    #[arg(long, default_value_t = default_regex_code_meta(), env)]
    #[serde(default = "default_regex_code_meta")]
    regex_code_meta: String,

    /// Regex to extract parms in meta
    #[arg(long, default_value_t = default_regex_meta_params(), env)]
    #[serde(default = "default_regex_meta_params")]
    regex_meta_params: String,

    /// Regex to extract refs in code
    #[arg(long, default_value_t = default_regex_code_refs(), env)]
    #[serde(default = "default_regex_code_refs")]
    regex_code_refs: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn default_log_level() -> String {
    "info".to_string()
}
fn default_filetype() -> String {
    "md".to_string()
}
fn default_regex_code_block() -> String {
    r#"(?:(?<=<!--\O*-->\O*)|(?<!<!--\O*))(?<backquotes>````*)(?<meta>.*)\n(?<content>\O*?)\n?\k<backquotes>"#.to_string()
}
fn default_regex_code_meta() -> String {
    r#"{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}"#.to_string()
}
fn default_regex_meta_params() -> String {
    r#"(?<key>[[:alnum:]]+)=(?<value>[^\s]+)"#.to_string()
}
fn default_regex_code_refs() -> String {
    r#"(?<indent>^[\t ]*|)<<(?<ref>[^\s]+)>>"#.to_string()
}

impl Cli {
    pub fn new() -> anyhow::Result<Self> {
        let command = Cli::command();
        let matches = command.get_matches();
        let cli = Cli::from_arg_matches(&matches)?;

        if let Some(config_path) = &cli.config {
            let settings = Config::builder()
                .add_source(config::File::with_name(
                    config_path,
                ))
                .build()?;
            let mut config =
                settings.try_deserialize::<Cli>()?;
            let mut matches_without_defaults =
                matches.clone();

            for arg_id in matches.ids() {
                let key = arg_id.as_str();
                if matches.value_source(key).unwrap()
                    == ValueSource::DefaultValue
                    && key != "Cli"
                {
                    matches_without_defaults
                        .try_clear_id(key)?;
                }
            }

            config.update_from_arg_matches(
                &matches_without_defaults,
            )?;

            Ok(config)
        } else {
            Ok(cli)
        }
    }
}

#[derive(Subcommand, Debug, Deserialize)]
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

#[derive(Debug)]
enum IlliterateBlock<'lang, 'a> {
    Ref {
        lang: RegexMatchWithString<'lang>,
        name: RegexMatchWithString<'a>,
    },
    File {
        lang: RegexMatchWithString<'lang>,
        path: RegexMatchWithString<'a>,
    },
}

#[derive(Debug)]
struct IlliterateCodeRef<'a> {
    indent_count: u32,
    indent_type: Indent,
    regex_match: RegexMatchWithString<'a>,
}

#[derive(Debug)]
enum Indent {
    Tabs(u8),
    Spaces(u8),
}

#[derive(Debug, Copy, Clone)]
struct RegexMatchWithString<'a> {
    content: &'a str,
    regex_match: fancy_regex::Match<'a>,
    content_start_offset: usize,
    content_end_offset: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::new()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_str(&cli.log_level)?)
        .init();

    let code_block_regex =
        fancy_regex::Regex::new(&cli.regex_code_block)?;
    let meta_regex =
        fancy_regex::Regex::new(&cli.regex_code_meta)?;
    let param_regex =
        fancy_regex::Regex::new(&cli.regex_meta_params)?;
    let ref_regex =
        fancy_regex::Regex::new(&cli.regex_code_refs)?;

    let walker = WalkDir::new(cli.source).into_iter();
    for entry in walker
        .filter_entry(|e| {
            is_dir(e) || is_filetype(e, &cli.filetype)
        })
        .filter_map(|e| e.ok())
        .filter(|e| is_filetype(e, &cli.filetype))
    {
        let file = entry.path().display().to_string();
        let content = read_to_string(&file)?;

        println!("{}", entry.path().display());
        for (params, refs_in_code) in code_block_regex
            .captures_iter(&content)
            .filter_map(Result::ok)
            .map(|cap| -> anyhow::Result<_> {
                Ok((
                    regex_match_from_capture(
                        &cap, "meta", 0,
                    )?,
                    regex_match_from_capture(
                        &cap, "content", 0,
                    )?,
                ))
            })
            .filter_map(Result::ok)
            .map(|(code_meta, code_content)| -> anyhow::Result<_> {
                let (_meta_lang, meta_params) =
                    meta_from_code_match(
                        &code_meta,
                        &meta_regex,
                        "lang",
                        "params",
                    )?;

                Ok((
                    meta_params,
                    refs_from_code_match(&code_content, &ref_regex, "ident", "ref"),
                ))
            })
            .filter_map(Result::ok)
            .map(|(meta_params, refs_in_code)| {
                let params = params_from_meta_match(&meta_params, &param_regex, "key", "value").collect::<HashMap<_, _>>();
                (params, refs_in_code.collect::<Vec<_>>())
            }) {

                dbg!(params);
                dbg!(refs_in_code);
            }
    }

    // dbg!(refs_in_files);

    Ok(())
}

fn regex_match_from_capture<'a>(
    capture: &fancy_regex::Captures<'a>,
    capture_field_name: &str,
    capture_offset: usize,
) -> anyhow::Result<RegexMatchWithString<'a>> {
    capture.name(capture_field_name).map(|m| RegexMatchWithString {
        content: m.as_str(),
        content_start_offset: capture_offset + m.start(),
        content_end_offset: capture_offset + m.end(),
        regex_match: m,
    }).ok_or_else(|| {
        anyhow::anyhow!("no {capture_field_name} field found on capture")
    })
}

fn meta_from_code_match<'a>(
    code_match: &RegexMatchWithString<'a>,
    meta_regex: &fancy_regex::Regex,
    lang_field_name: &str,
    params_field_name: &str,
) -> anyhow::Result<(
    RegexMatchWithString<'a>,
    RegexMatchWithString<'a>,
)> {
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
            anyhow::anyhow!("couldn't extract {lang_field_name} and {params_field_name} from code_meta")
        })
}

fn refs_from_code_match<'a>(
    code_match: &RegexMatchWithString<'a>,
    ref_regex: &'a fancy_regex::Regex,
    ident_field_name: &'a str,
    ref_field_name: &'a str,
) -> impl Iterator<Item = IlliterateCodeRef<'a>> + use<'a> {
    // We make a clone of usize (copy) so the closure can use it
    // while releasing the borrow at the end of the function
    let start_offset =
        code_match.content_start_offset.clone();

    ref_regex
        .captures_iter(code_match.content)
        .filter_map(Result::ok)
        .map(move |cap| -> anyhow::Result<_> {
            let _indent_match = regex_match_from_capture(
                &cap,
                ident_field_name,
                start_offset,
            )?;
            let ref_match = regex_match_from_capture(
                &cap,
                ref_field_name,
                start_offset,
            )?;

            Ok(IlliterateCodeRef {
                indent_count: 0,
                indent_type: Indent::Spaces(2),
                regex_match: ref_match,
            })
        })
        .filter_map(Result::ok)
}

fn params_from_meta_match<'a>(
    meta_match: &RegexMatchWithString<'a>,
    param_regex: &'a fancy_regex::Regex,
    key_field_name: &'a str,
    value_field_name: &'a str,
) -> impl Iterator<Item = (&'a str, RegexMatchWithString<'a>)>
+ use<'a> {
    // We make a clone of usize (copy) so the closure can use it
    // while releasing the borrow at the end of the function
    let start_offset =
        meta_match.content_start_offset.clone();

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
}
