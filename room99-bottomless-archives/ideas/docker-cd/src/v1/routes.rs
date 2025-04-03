use axum::{
    routing::{delete, post},
    Router,
};
use eyre::Result;

use super::controllers;

#[tracing::instrument]
pub fn deploy_router() -> Result<Router> {
    Ok(Router::new()
        .route("/", post(controllers::deploy::create_container))
        .route(
            "/:container_name",
            delete(controllers::deploy::stop_container),
        ))
}
