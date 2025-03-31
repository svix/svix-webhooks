pub use omniqueue::QueueError;
use svix_bridge_types::svix;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("queue error: {0}")]
    Queue(#[from] QueueError),
    #[error("svix API error: {0}")]
    Svix(#[from] svix::error::Error),
    #[error("{0}")]
    Generic(String),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Json(e) => std::io::Error::other(e),
            Error::Queue(e) => std::io::Error::other(e),
            Error::Svix(e) => std::io::Error::other(e),
            Error::Generic(e) => std::io::Error::other(e),
        }
    }
}
