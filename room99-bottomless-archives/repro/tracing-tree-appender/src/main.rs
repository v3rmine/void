use std::io;

use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_tree::HierarchicalLayer;

fn main() {
    let file_appender = tracing_appender::rolling::hourly("./", "log");

    let registry = Registry::default()
        .with(HierarchicalLayer::default().with_writer(io::stdout))
        .with(HierarchicalLayer::default().with_writer(file_appender));

    tracing::subscriber::set_global_default(registry).unwrap();

    tracing::span!(Level::WARN, "main").in_scope(|| {
        tracing::warn!("Hello world!");
    });
}
