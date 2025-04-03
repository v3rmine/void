use std::{env, io};

use eyre::Result;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_logger() -> Result<Option<WorkerGuard>> {
    // This will print tracing events to standard output for humans to read
    let logger = tracing_subscriber::registry()
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .with(fmt::layer().with_writer(io::stdout).pretty());
    // When this variable goes out of scope (at the end of the function where this function is called), it will flush the log file writer
    let mut file_logger_guard = Option::None;

    // Masking the inner type using "dyn" keyword because return types are differents in the if / else
    let logger: Box<dyn Subscriber + Send + Sync + 'static> =
        if let Ok(directory) = env::var("LOG_DIRECTORY") {
            if !directory.is_empty() {
                let file_appender = tracing_appender::rolling::hourly(directory, "docker-cd.log");
                let (log_writer, guard) = tracing_appender::non_blocking(file_appender);
                file_logger_guard = Some(guard);

                Box::new(
                    logger.with(
                        fmt::layer()
                            .with_writer(log_writer)
                            .with_ansi(false)
                            .compact(),
                    ),
                )
            } else {
                Box::new(logger)
            }
        } else {
            Box::new(logger)
        };

    logger.init();

    Ok(file_logger_guard)
}
