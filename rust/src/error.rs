// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt;

use http::StatusCode;
use http_body_util::BodyExt;
use hyper::body::Incoming;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Svix API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Http Error
    Http(HttpErrorContent<crate::models::HttpErrorOut>),
    /// Http Validation Error
    Validation(HttpErrorContent<crate::models::HttpValidationError>),
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(err.to_string())
    }

    pub(crate) async fn from_response(status_code: StatusCode, body: Incoming) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if status_code == StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: status_code,
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

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        Self::generic(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::generic(err)
    }
}

#[derive(Debug, Clone)]
pub struct HttpErrorContent<T> {
    pub status: http::StatusCode,
    pub payload: Option<T>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Error::Http(e) => format!("Http error (status={}) {:?}", e.status, e.payload).fmt(f),
            Error::Validation(e) => format!("Validation error {:?}", e.payload).fmt(f),
        }
    }
}

impl std::error::Error for Error {}
