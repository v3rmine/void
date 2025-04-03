#[cfg(feature = "log_file")]
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter, Layer};
#[cfg(feature = "tree")]
use tracing_tree::HierarchicalLayer;

/// Add a pretty-printed log layer to the subscriber.
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

/// Add an environment filter based on the LOG_LEVEL env variable to the subscriber.
pub fn with_env<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    EnvFilter::from_env("LOG_LEVEL").boxed()
}

/// Add a hierarchical layer to the subscriber (to see the different spans levels).
#[cfg(feature = "tree")]
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

/// Add an honeycomb reporting layer to the subscriber.
#[cfg(feature = "honeycomb")]
pub fn with_honeycomb<S>(
    service_name: &'static str,
    dataset: &str,
) -> Option<Box<dyn Layer<S> + Send + Sync + 'static>>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Ok(api_key) = std::env::var("HONEYCOMB_API_KEY") {
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

/// Add a file (using the LOG_DIRECTORY env variable) logging output to the subscriber.
#[cfg(feature = "log_file")]
pub fn with_logfiles<S>(
    logfile_prefix: &str,
) -> Option<(Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Ok(directory) = std::env::var("LOG_DIRECTORY") {
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
