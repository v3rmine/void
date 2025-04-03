use axum::Router;
use eyre::Result;

mod controllers;
mod routes;

#[tracing::instrument]
pub fn router() -> Result<Router> {
    Ok(Router::new().nest("/deploy", routes::deploy_router()?))
}
