// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{fmt, time::Duration};

use http::StatusCode;
use http_body_util::BodyExt;
use hyper::body::Incoming;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Svix API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// A timeout, with the elapsed time and configured timeout value if known
    Timeout {
        elapsed: Option<Duration>,
        timeout: Option<Duration>,
    },
    /// Http Error
    Http(HttpErrorContent<crate::models::HttpErrorOut>),
    /// Http Validation Error
    Validation(HttpErrorContent<crate::models::HttpValidationError>),
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(format!("{err:?}"))
    }

    pub(crate) async fn from_response(
        status_code: StatusCode,
        body: Incoming,
        timeout: Option<Duration>,
    ) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if status_code == StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: StatusCode::UNPROCESSABLE_ENTITY,
                        payload: serde_json::from_slice(&bytes).ok(),
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: status_code,
                        payload: serde_json::from_slice(&bytes).ok(),
                    })
                }
            }
            Err(e) if e.is_timeout() => Self::Timeout {
                // hyper doesn't give us the elapsed time by default
                elapsed: None,
                timeout,
            },
            Err(e) => Self::Generic(e.to_string()),
        }
    }
}

// TODO: Remove for v2.0 of the library (very uncommon impl for an error type)
impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Self::Timeout {
                elapsed: Some(elapsed),
                timeout: Some(timeout),
            } => format!("Request timeout after {elapsed:?} (timeout {timeout:?})").fmt(f),
            Self::Timeout {
                timeout: Some(timeout),
                ..
            } => format!("Request timeout (threshold {timeout:?})").fmt(f),
            Self::Timeout { .. } => "Request timeout".fmt(f),
            Error::Http(e) => format!("Http error (status={}) {:?}", e.status, e.payload).fmt(f),
            Error::Validation(e) => format!("Validation error {:?}", e.payload).fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct HttpErrorContent<T> {
    pub status: StatusCode,
    pub payload: Option<T>,
}
