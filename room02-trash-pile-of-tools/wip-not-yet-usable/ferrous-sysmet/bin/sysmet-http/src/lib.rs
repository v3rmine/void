use axum::{
    extract::{Extension, Query},
    routing::get,
    Router, Server,
};
pub use color_eyre::Result;
use include_dir::{include_dir, Dir};
use log::{debug, info, trace, tracing};
use maud::{html, Markup};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

mod components;
pub use components::*;
pub(crate) mod generator;
pub(crate) mod macros;
pub(crate) mod svg;

use generator::ChartsData;

pub(crate) const SOURCE_URL: &str = "https://github.com/joxcat/sysmet";
pub(crate) const WEBSITE_TITLE: &str = "Ferrous System Metrics";

pub(crate) const CSS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/css/exports");
pub(crate) static CSS_HASHES: Lazy<HashMap<String, (PathBuf, String)>> =
    generate_hashes!(CSS_HASHES, CSS_DIR);
static_files_server!(css_assets, CSS_DIR, CSS_HASHES, "text/css");

#[tracing::instrument]
pub async fn run_server(addr: SocketAddr, database: &str) -> Result<()> {
    let chart_data = RwLock::new(ChartsData::default());
    let shared_chart_data = Arc::new(chart_data);

    let (db_tx, db_rx) = tokio::sync::oneshot::channel::<()>();
    let (server_tx, server_rx) = tokio::sync::oneshot::channel::<()>();
    let handle = {
        let shared_chart_data = shared_chart_data.clone();
        let database = database.to_string();

        tokio::spawn(generator::actualization_task(
            shared_chart_data,
            database,
            db_rx,
        ))
    };

    {
        tokio::spawn(async move {
            debug!("Spawned Ctrl-C handler task");
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
            trace!("Catched Ctrl-C");

            db_tx
                .send(())
                .expect("Failed to send stop signal to the actualization task");
            trace!("Waiting for the actualization task to end");
            handle
                .await
                .expect("Error while waiting for the end of the actualization task");
            trace!("Actualization task ended");
            server_tx
                .send(())
                .expect("Failed to send stop signal to the server task");

            debug!("Finished Ctrl-C gracefull shutdown");
        });
    }

    let app = Router::new()
        .route("/", get(home))
        .route("/css/:path", get(css_assets))
        .layer(Extension(shared_chart_data));

    info!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            server_rx.await.ok();
            trace!("Received server stop signal");
        })
        .await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct HomeQuery {
    t: Option<String>,
    refresh: Option<String>,
}

#[tracing::instrument]
async fn home(
    time_from_now: Query<HomeQuery>,
    Extension(chart_data): Extension<Arc<RwLock<ChartsData>>>,
) -> Markup {
    let time_from_now = time_from_now.0;
    let _time = time_from_now // TODO: Filter results directly on the svg
        .t
        .clone()
        .and_then(|ref t| humantime::parse_duration(t).ok());
    let refresh = time_from_now.refresh.is_some() && time_from_now.refresh.clone().unwrap() == "on";

    let chart_sections = {
        let data = chart_data.read().await;
        data.metrics.clone()
    };

    Base(
        BaseContext::builder().refresh_every_minute(refresh).build(),
        html! {
            section {
                h1 { "sysmet faster" }
                form {
                    div {
                        label {
                            span { "Time range:" }
                            input name="t" value=(time_from_now.t.unwrap_or_else(|| "3h0m0s".to_string()));
                            span { "ago to now." }
                        }
                        label {
                            @if refresh {
                                input type="checkbox" name="refresh" checked;
                            } @else {
                                input type="checkbox" name="refresh";
                            }
                            span { "Auto-refresh every minute" }
                        }
                    }
                    input type="submit" { "Change" }
                }
            }
            section {
                @for (title, context) in chart_sections {
                    section {
                        h2 { (title) }
                        (Chart(context))
                    }
                }
            }
            section {
                a href=(SOURCE_URL) referer="none" target="_blank" { "Source code" }
                span { " - Licensed under the AGPL v3.0." }
            }
        },
    )
}
