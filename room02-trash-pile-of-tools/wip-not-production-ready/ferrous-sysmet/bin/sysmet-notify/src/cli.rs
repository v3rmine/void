use std::time::Duration;

use clap::Parser;
use clap_verbosity_flag::Verbosity;
use lettre::message::Mailbox;
use log::{trace, tracing};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        long,
        env = "CPU_THRESHOLD",
        default_value = "95",
        value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max CPU Usage before warning"
    )]
    pub cpu_threshold: Option<u32>,
    #[clap(
        long,
        env = "RAM_THRESHOLD",
        default_value = "90",
        value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max RAM Usage before warning"
    )]
    pub ram_threshold: Option<u32>,
    #[clap(
        long,
        env = "SWAP_THRESHOLD",
        default_value = "65",
        value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max Swap Usage before warning"
    )]
    pub swap_threshold: Option<u32>,
    #[clap(
        long,
        env = "MEMORY_THRESHOLD",
        default_value = "75",
		value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max Memory (RAM & Swap) Usage before warning",
		conflicts_with_all = ["ram_threshold", "swap_threshold"]
    )]
    pub memory_threshold: Option<u32>,
    #[clap(
        long,
        env = "DISK_THRESHOLD",
        default_value = "85",
        value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max Disk Usage before warning"
    )]
    pub disk_threshold: Option<u32>,
    #[clap(
        long,
        env = "AVG_LOAD_THRESHOLD",
        default_value = "85",
        value_name = "PERCENTAGE",
		value_parser = clap::value_parser!(u32).range(0..=100),
        help = "Max Average Load before warning"
    )]
    pub avg_load_threshold: Option<u32>,
    #[clap(
		short,
		long = "from",
		env = "MAIL_FROM",
		value_parser = mailbox_try_from_str,
		required_unless_present("dry_run"),
		help = "Identity that will be used to send the mail"
	)]
    pub from: Option<Mailbox>,
    #[clap(
		short,
		long = "contacts",
		env = "MAIL_CONTACTS",
		value_delimiter = ',',
		value_parser = mailbox_try_from_str,
		required_unless_present("dry_run"),
		help = "Contacts that will receive the mail",
		action = clap::ArgAction::Append
	)]
    pub contacts: Vec<Mailbox>,
    #[clap(
        long = "cooldown",
        env = "MAIL_COOLDOWN",
        default_value = "1h",
		value_parser = duration_try_from_str,
        help = "Time to wait before sending a mail again"
    )]
    pub cooldown: Duration,
    #[clap(
        long = "smtp-user",
        env = "SMTP_USER",
        required_unless_present("dry_run"),
        help = "SMTP Username to authenticate with the Relay"
    )]
    pub smtp_user: Option<String>,
    #[clap(
        long = "smtp-pass",
        env = "SMTP_PASSWORD",
        required_unless_present("dry_run"),
        help = "SMTP Password to authenticate with the Relay"
    )]
    pub smtp_password: Option<String>,
    #[clap(
        long = "smtp-relay",
        env = "SMTP_RELAY",
        required_unless_present("dry_run"),
        help = "SMTP Relay that will be used to send the mail"
    )]
    pub smtp_relay: Option<String>,
    #[clap(
        long = "smtp-port",
        env = "SMTP_PORT",
		value_parser = clap::value_parser!(u16).range(1..=65535),
		default_value = "465",
        help = "SMTP Relay port that will be used to connect to the relay"
    )]
    pub smtp_port: u16,
    #[clap(
        long = "last-sent-path",
        env = "LAST_SENT_PATH",
        default_value = "/tmp/sysmet-notify-last-mail.txt",
        help = "Timestamp of the last time a mail was sent"
    )]
    pub last_sent_instant: Option<String>,
    #[clap(
        long = "env",
        default_value = ".env",
        help = "Path to the optional env file",
        required = false
    )]
    pub env_path: String,
    #[clap(long = "dry-run", help = "Simulate the run")]
    pub dry_run: bool,
    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[tracing::instrument(level = "trace")]
fn mailbox_try_from_str(value: &str) -> Result<Mailbox, lettre::address::AddressError> {
    let result = value.parse::<Mailbox>();
    trace!(parsed_mailbox =? result);
    result
}

#[tracing::instrument(level = "trace")]
fn duration_try_from_str(value: &str) -> Result<Duration, humantime::DurationError> {
    let result = humantime::parse_duration(value);
    trace!(parsed_duration =? result);
    result
}
