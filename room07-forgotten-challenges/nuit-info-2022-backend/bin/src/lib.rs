#![forbid(unsafe_code)]

use axum::{Router, Server};
pub use eyre::{Error, Result};
use logging::{debug, info, trace, tracing};
use std::net::SocketAddr;

pub(crate) mod macros;
pub(crate) mod nostalgia;

#[tracing::instrument]
pub async fn run_server(addr: SocketAddr) -> Result<()> {
    let (server_tx, server_rx) = tokio::sync::oneshot::channel::<()>();

    // NOTE: This is the graceful shutdown handler
    {
        tokio::spawn(async move {
            debug!("Spawned Ctrl-C handler task");
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
            trace!("Catched Ctrl-C");
            server_tx
                .send(())
                .expect("Failed to send stop signal to the server task");

            debug!("Finished Ctrl-C gracefull shutdown");
        });
    }

    let app = Router::new();

    info!("Listening on {}", addr);
    // NOTE: This is the server handler
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            server_rx.await.ok();
            trace!("Received server stop signal");
        })
        .await?;

    Ok(())
}
