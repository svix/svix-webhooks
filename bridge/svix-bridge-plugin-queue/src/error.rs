pub use omniqueue::QueueError;
use svix_bridge_types::svix;

pub enum Error {
    Payload(String),
    Json(serde_json::Error),
    Queue(QueueError),
    Svix(svix::error::Error),
    Generic(String),
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<svix::error::Error> for Error {
    fn from(value: svix::error::Error) -> Self {
        Error::Svix(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}

impl From<QueueError> for Error {
    fn from(value: QueueError) -> Self {
        Error::Queue(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Payload(e) => std::io::Error::new(std::io::ErrorKind::Other, e),
            Error::Json(e) => std::io::Error::new(std::io::ErrorKind::Other, e),
            Error::Queue(e) => std::io::Error::new(std::io::ErrorKind::Other, e),
            Error::Svix(e) => std::io::Error::new(std::io::ErrorKind::Other, e),
            Error::Generic(e) => std::io::Error::new(std::io::ErrorKind::Other, e),
        }
    }
}
