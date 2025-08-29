#![forbid(unsafe_code)]
use axum::{Router, Server};
pub use eyre::Result;
use log::tracing::info;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env::setup_env()?;
    let _logger_guard = log::setup_logger();

    let app = Router::new();

    let host = env::var_not_empty("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var_not_empty("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = [host, ":".to_string(), port]
        .concat()
        .parse::<SocketAddr>()?;

    info!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
