#![forbid(unsafe_code)]
use either::{for_both, Either};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Re-export tracing for convenience.
pub use tracing;

/// Re-export log macros for convenience.
pub use tracing::{debug, error, info, trace, warn};

pub mod layers;

pub fn setup_simple_logger() {
    // This will print tracing events to standard output for humans to read
    tracing_subscriber::Registry::default()
        .with(layers::with_env())
        .with(layers::with_pretty())
        .init();
}

pub fn setup_hierarchical_logger() {
    // This will print tracing events to standard output for humans to read
    tracing_subscriber::Registry::default()
        .with(layers::with_env())
        .with(layers::with_hierarchical())
        .init();
}

pub fn setup_logger_with_logfiles(logfile_prefix: &str) -> Option<WorkerGuard> {
    // This will print tracing events to standard output for humans to read
    let logger = tracing_subscriber::Registry::default()
        .with(layers::with_env())
        .with(layers::with_hierarchical());
    // When this variable goes out of scope (at the end of the function where this function is called), it will flush the log file writer
    let mut file_logger_guard = Option::None;

    // Masking the inner type using "dyn" keyword because return types are differents in the if / else
    let logger = if let Some((layer, guard)) = layers::with_logfiles(logfile_prefix) {
        file_logger_guard = Some(guard);
        Either::Left(logger.with(layer))
    } else {
        Either::Right(logger)
    };

    for_both!(logger, logger => logger.init());
    file_logger_guard
}
