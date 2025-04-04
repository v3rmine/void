use tracing_subscriber::fmt::format::FmtSpan;

pub fn setup_tracing() {
    tracing_subscriber::fmt::fmt()
        .compact()
        .with_test_writer()
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter("debug")
        .init();
}
