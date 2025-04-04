use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to serialize the content")]
    Serialization(serde_json::Error),
    #[error("failed to deserialize the content")]
    Desrialization(serde_json::Error),
    #[error("http error")]
    Http(#[from] hyper::http::Error),
    #[error("hyper error")]
    Hyper(#[from] hyper::Error),
}
