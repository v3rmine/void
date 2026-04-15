use std::env;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter, Layer,
};
use tracing_tree::HierarchicalLayer;

pub fn with_pretty<S>() -> impl Layer<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_line_number(false)
        .with_file(false)
        .with_thread_names(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .without_time()
}

pub fn with_env<S>() -> impl Layer<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::from_env("LOG_LEVEL")
}

pub fn with_hierarchical<S>() -> impl Layer<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    HierarchicalLayer::new(3)
        .with_bracketed_fields(true)
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_targets(true)
}

pub fn with_logfiles<S>(logfile_prefix: &str) -> Option<(impl Layer<S>, WorkerGuard)>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Ok(directory) = env::var("LOG_DIRECTORY") {
        if !directory.is_empty() {
            let file_appender =
                tracing_appender::rolling::hourly(directory, format!("{logfile_prefix}.log"));
            let (log_writer, guard) = tracing_appender::non_blocking(file_appender);
            Some((
                fmt::layer()
                    .with_writer(log_writer)
                    .with_ansi(false)
                    .compact(),
                guard,
            ))
        } else {
            None
        }
    } else {
        None
    }
}
