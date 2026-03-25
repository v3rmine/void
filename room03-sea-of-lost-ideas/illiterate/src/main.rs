use std::{
    collections::HashMap, fs::read_to_string,
    path::PathBuf, str::FromStr,
};

use clap::{Parser, Subcommand, ValueHint};
use confique::Config;
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
    //#[arg(short, long, default_value_t = default_log_level(), env)]
    //#[serde(default = "default_log_level")]
    log_level: String,

    /// Filetype of the source files
    #[config(default = "md", layer_attr(arg(long, env)))]
    //#[arg(long, default_value_t = default_filetype(), env)]
    //#[serde(default = "default_filetype")]
    filetype: String,

    /// Regex to extract code blocks
    #[config(
        default = r#"(?m)(?:(?<=<!--\O*-->\O*)|(?<!<!--\O*))(?<backquotes>^````*+)(?<meta>.*)\n(?<content>\O*?)\n?\k<backquotes>"#,
        layer_attr(arg(long, env))
    )]
    //#[arg(long, default_value_t = default_regex_code_block(), env)]
    //#[serde(default = "default_regex_code_block")]
    regex_code_block: String,

    /// Regex to extract meta from code blocks
    #[config(
        default = r#"{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}"#,
        layer_attr(arg(long, env))
    )]
    //#[arg(long, default_value_t = default_regex_code_meta(), env)]
    //#[serde(default = "default_regex_code_meta")]
    regex_code_meta: String,

    /// Regex to extract parms in meta
    #[config(
        default = r#"(?<key>[[:alnum:]]+)=(?<value>[^\s]+)"#,
        layer_attr(arg(long, env))
    )]
    //#[arg(long, default_value_t = default_regex_meta_params(), env)]
    //#[serde(default = "default_regex_meta_params")]
    regex_meta_params: String,

    /// Regex to extract refs in code
    #[config(
        default = r#"(?m)^(?:(?![\t ]+<<)(?<line_indent>[\t ]*+)(?:[^\n<]*+|<)+|(?<direct_indent>[\t ]*))<<(?<ref>[^>]+)>>"#,
        layer_attr(arg(long, env))
    )]
    //#[arg(long, default_value_t = default_regex_code_refs(), env)]
    //#[serde(default = "default_regex_code_refs")]
    regex_code_refs: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_hint = ValueHint::FilePath, env)]
    config: Option<PathBuf>,

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
        if let Some(config_file) = cli.config {
            config = config.file(config_file);
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

    let code_block_regex =
        fancy_regex::Regex::new(&cli.regex_code_block)?;
    let meta_regex =
        fancy_regex::Regex::new(&cli.regex_code_meta)?;
    let param_regex =
        fancy_regex::Regex::new(&cli.regex_meta_params)?;
    let ref_regex =
        fancy_regex::Regex::new(&cli.regex_code_refs)?;

    let walker = WalkDir::new(source_dir).into_iter();
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
                    refs_from_code_match(&code_content, &ref_regex, "direct_indent", "line_indent", "ref"),
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
    direct_indent_field_name: &'a str,
    line_indent_field_name: &'a str,
    ref_field_name: &'a str,
) -> impl Iterator<Item = IlliterateCodeRef<'a>> + use<'a> {
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
}

fn params_from_meta_match<'a>(
    meta_match: &RegexMatchWithString<'a>,
    param_regex: &'a fancy_regex::Regex,
    key_field_name: &'a str,
    value_field_name: &'a str,
) -> impl Iterator<Item = (&'a str, RegexMatchWithString<'a>)>
+ use<'a> {
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
}
