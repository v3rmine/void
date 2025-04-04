#[cfg(feature = "file")]
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter, Layer,
};
#[cfg(feature = "tree")]
use tracing_tree::HierarchicalLayer;

/// Add a pretty-printed log layer to the subscriber.
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
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
}

/// Add an environment filter based on the `LOG_LEVEL` env variable to the subscriber.
pub fn with_env<S>() -> impl Layer<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::from_env("LOG_LEVEL")
}

/// Add a hierarchical layer to the subscriber (to see the different spans levels).
#[cfg(feature = "tree")]
pub fn with_hierarchical<S>(indent_amount: usize) -> impl Layer<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    HierarchicalLayer::new(indent_amount)
        .with_bracketed_fields(true)
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_targets(true)
}

/// Add a file logging output to the subscriber.
#[cfg(feature = "file")]
pub fn with_logfiles<S>(
    logfile_prefix: &str,
    directory: &str,
) -> Option<(impl Layer<S>, WorkerGuard)>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
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
}