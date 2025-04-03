#![allow(clippy::dbg_macro, clippy::todo)]
use axum::{routing::get, Extension, Json, Router};
use dotenvy::dotenv;
use eyre::Result;
use sea_orm::Database;
use serde_json::json;
use services::env::var_not_empty;
use tower_http::trace::TraceLayer;

use crate::services::{env::setup_env, logger};

mod macros;
mod services;
mod v1;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Setup the error handlers
    color_eyre::install()?;
    setup_env()?;

    // Load environment variables from .env file
    dotenv().ok();
    // NOTE: When this variable goes out of scope (at the end of this function), it will flush the log file writer
    let _file_writer_guard = logger::setup_logger()?;

    let database_connection = Database::connect(var_not_empty("DATABASE_URL")?).await?;

    let _app = Router::new()
        .route(
            "/api",
            get(|| async { Json(json!({"message": "Welcome to REST api!"})) }),
        )
        .nest("/v1", v1::router()?)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(database_connection));

    //let port = var_not_empty("PORT").unwrap_or_else(|_| "8888".to_string());

    let docker = services::docker::DockerConnection::default();
    dbg!(docker.list_containers().await?);

    //info!("Magic happens on port {port}");
    /*axum::Server::bind(&format!("0.0.0.0:{port}").parse()?)
    .serve(app.into_make_service())
    .await?;**/

    Ok(())
}
