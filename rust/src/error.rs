// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::error;
use std::fmt;

use http::status;

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

#[derive(Debug, Clone)]
pub struct HttpErrorContent<T> {
    pub status: reqwest::StatusCode,
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

impl From<Error> for String {
    fn from(err: Error) -> String {
        err.to_string()
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl<T> From<crate::apis::Error<T>> for Error {
    fn from(err: crate::apis::Error<T>) -> Error {
        match err {
            crate::apis::Error::ResponseError(e) => {
                if e.status == status::StatusCode::UNPROCESSABLE_ENTITY {
                    Error::Validation(HttpErrorContent {
                        status: e.status,
                        payload: serde_json::from_str(&e.content).ok(),
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: e.status,
                        payload: serde_json::from_str(&e.content).ok(),
                    })
                }
            }
            _ => Error::Generic(err.to_string()),
        }
    }
}
