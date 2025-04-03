use axum::{extract::Path, response::IntoResponse};
use tracing::log::warn;

#[tracing::instrument]
pub async fn create_container() -> impl IntoResponse {
    warn!("unimplemented!");
    "Deploying container..."
}

#[tracing::instrument(fields(container_name = %container_name))]
pub async fn stop_container(Path(container_name): Path<String>) -> impl IntoResponse {
    warn!("unimplemented!");
    "Stopping container..."
}
