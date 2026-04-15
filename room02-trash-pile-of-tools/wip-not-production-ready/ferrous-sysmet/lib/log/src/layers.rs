use std::env;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter, Layer};
use tracing_tree::HierarchicalLayer;

pub fn with_pretty<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_line_number(false)
        .with_file(false)
        .boxed()
}

pub fn with_env<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::from_env("LOG_LEVEL").boxed()
}

pub fn with_hierarchical<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    HierarchicalLayer::new(3)
        .with_bracketed_fields(true)
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_targets(true)
        .boxed()
}

pub fn with_honeycomb<S>(
    service_name: &'static str,
    dataset: &str,
) -> Option<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Ok(api_key) = env::var("HONEYCOMB_API_KEY") {
        let honeycomb_config = libhoney::Config {
            options: libhoney::client::Options {
                api_key,
                dataset: dataset.to_string(),
                ..libhoney::client::Options::default()
            },
            transmission_options: libhoney::transmission::Options::default(),
        };

        let telemetry_layer =
            tracing_honeycomb::new_honeycomb_telemetry_layer(service_name, honeycomb_config);

        Some(telemetry_layer.boxed())
    } else {
        None
    }
}

pub fn with_logfiles<S>(
    logfile_prefix: &str,
) -> Option<(Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)>
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
                    .compact()
                    .boxed(),
                guard,
            ))
        } else {
            None
        }
    } else {
        None
    }
}
