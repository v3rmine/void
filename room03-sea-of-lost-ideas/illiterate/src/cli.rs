use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use confique::Config;

#[derive(Config, Debug, Clone)]
#[config(layer_attr(derive(clap::Args, Debug)))]
pub struct CliConfig {
    /// Log level
    #[config(
        default = "info",
        layer_attr(arg(short, long, env))
    )]
    pub log_level: String,

    /// Filetype of the source files
    #[config(default = "md", layer_attr(arg(long, env)))]
    pub filetype: String,

    /// Params name field identifier
    /// Used by named code blocks
    #[config(default = "name", layer_attr(arg(long, env)))]
    pub params_name_key: String,

    /// Params file field identifier
    /// Used by file code blocks
    #[config(default = "file", layer_attr(arg(long, env)))]
    pub params_file_key: String,

    /// Regex to extract code blocks
    #[config(
        default = r#"(?m)(?:(?<=<!--\O*-->\O*)|(?<!<!--\O*))(?<backquotes>^````*+)(?<meta>.*)\n(?<content>\O*?)\n?\k<backquotes>"#,
        layer_attr(arg(long, env))
    )]
    pub regex_code_block: String,

    /// Regex to extract meta from code blocks
    #[config(
        default = r#"{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}"#,
        layer_attr(arg(long, env))
    )]
    pub regex_code_meta: String,

    /// Regex to extract parms in meta
    #[config(
        default = r#"(?<key>[[:alnum:]]+)=(?<value>[^\s]+)"#,
        layer_attr(arg(long, env))
    )]
    pub regex_meta_params: String,

    /// Regex to extract refs in code
    #[config(
        default = r#"(?m)^(?:(?![\t ]+<<)(?<line_indent>[\t ]*+)(?:[^\n<]*+|<)+|(?<direct_indent>[\t ]*))(?<full_ref><<(?<ref>[^>]+)>>)"#,
        layer_attr(arg(long, env))
    )]
    pub regex_code_refs: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Use one or multiple configs files
    #[arg(short = 'c', long = "config", value_hint = ValueHint::FilePath, env)]
    pub config_files: Vec<PathBuf>,

    /// Source dir
    #[arg(short, long, value_hint = ValueHint::DirPath, env)]
    pub source: String,

    #[command(flatten)]
    pub cli_config: <CliConfig as Config>::Layer,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn build()
    -> anyhow::Result<(String, CliConfig, Option<Commands>)>
    {
        let cli = Cli::parse();
        let mut config =
            CliConfig::builder().preloaded(cli.cli_config);
        for file in cli.config_files {
            config = config.file(file);
        }

        Ok((cli.source, config.load()?, cli.command))
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate references without emitting files
    Check,
    /// Resolve references and emit files
    Tangle {
        /// Output directory for generated files
        #[arg(long, default_value = "out")]
        outdir: String,
        /// Log file content to debug instead of writing to disk
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Watch source directory and tangle on change
    Watch {
        /// Output directory for generated files
        #[arg(long, default_value = "out")]
        outdir: String,
        /// Use polling instead of native file watcher
        #[arg(long, default_value_t = false)]
        poll: bool,
    },
}
