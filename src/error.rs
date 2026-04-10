use std::fmt;

use headers::ContentType;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Diom API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Http Error
    Http(HttpErrorContent<StandardHttpError>),
    /// Http Validation Error
    Validation(HttpErrorContent<HttpValidationError>),
}

fn deserialize_body<'b, 'a, T>(
    status_code: http::StatusCode,
    mime: &headers::Mime,
    bytes: &'b [u8],
) -> Option<T>
where
    T: Deserialize<'a>,
    'b: 'a,
{
    let payload = if mime.subtype() == "json" {
        serde_json::from_slice(bytes).ok()
    } else if mime.essence_str() == "application/msgpack" {
        rmp_serde::from_slice(bytes).ok()
    } else {
        None
    };
    if payload.is_none() {
        let as_str = String::from_utf8_lossy(bytes);
        tracing::warn!(?status_code, mime_type=?mime, response = %as_str, "unparsable error");
    }
    payload
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(format!("{err:?}"))
    }

    pub(crate) async fn from_response(
        status_code: http::StatusCode,
        body: Incoming,
        content_type: ContentType,
    ) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                let mime: headers::Mime = content_type.into();
                if status_code == http::StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: http::StatusCode::UNPROCESSABLE_ENTITY,
                        payload: deserialize_body(status_code, &mime, &bytes),
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: status_code,
                        payload: deserialize_body(status_code, &mime, &bytes),
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
