use std::fmt;

use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Coyote API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Http Error
    Http(HttpErrorContent<StandardHttpError>),
    /// Http Validation Error
    Validation(HttpErrorContent<HttpValidationError>),
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(format!("{err:?}"))
    }

    pub(crate) async fn from_response(status_code: http::StatusCode, body: Incoming) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if status_code == http::StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: http::StatusCode::UNPROCESSABLE_ENTITY,
                        payload: serde_json::from_slice(&bytes).ok(),
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: status_code,
                        payload: serde_json::from_slice(&bytes).ok(),
                    })
                }
            }
            Err(e) => Self::Generic(e.to_string()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Error::Http(e) => format!("Http error (status={}) {:?}", e.status, e.payload).fmt(f),
            Error::Validation(e) => format!("Validation error {:?}", e.payload).fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct HttpErrorContent<T> {
    pub status: http::StatusCode,
    pub payload: Option<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StandardHttpError {
    pub code: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HttpValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}
