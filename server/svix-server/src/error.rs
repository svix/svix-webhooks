// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::error;
use std::fmt;

use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use hyper::StatusCode;
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;

/// A short-hand version of a [`std::result::Result`] that always returns an Svix [Error].
pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Svix API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Database error
    Database(String),
    /// Queue error
    Queue(String),
    /// Database error
    Validation(String),
    /// Any kind of HttpError
    Http(HttpError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) | Error::Database(s) | Error::Queue(s) | Error::Validation(s) => {
                s.fmt(f)
            }
            Error::Http(s) => s.fmt(f),
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

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Generic(err)
    }
}

impl From<getrandom::Error> for Error {
    fn from(err: getrandom::Error) -> Error {
        Error::Generic(err.to_string())
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Error {
        if let DbErr::Query(ref err_str) = err {
            // Surely there's a better way
            if err_str.contains("duplicate key value violates unique constraint") {
                Error::Http(HttpError::conflict(None, None))
            } else {
                Error::Database(err.to_string())
            }
        } else {
            Error::Database(err.to_string())
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("{:?}", &self);
        match self {
            Error::Http(s) => s.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpErrorBody {
    code: String,
    detail: String,
}

#[derive(Debug, Clone)]
pub struct HttpError {
    status: StatusCode,
    body: HttpErrorBody,
}

impl HttpError {
    pub fn new(status: StatusCode, code: String, detail: String) -> Self {
        Self {
            status,
            body: HttpErrorBody { code, detail },
        }
    }

    pub fn bad_request(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            code.unwrap_or_else(|| "generic_error".to_owned()),
            detail.unwrap_or_else(|| "Generic error".to_owned()),
        )
    }

    pub fn not_found(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            code.unwrap_or_else(|| "not_found".to_owned()),
            detail.unwrap_or_else(|| "Entity not fonud".to_owned()),
        )
    }

    pub fn unauthorized(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            code.unwrap_or_else(|| "authentication_failed".to_owned()),
            detail.unwrap_or_else(|| "Incorrect authentication credentials.".to_owned()),
        )
    }

    pub fn conflict(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            code.unwrap_or_else(|| "conflict".to_owned()),
            detail.unwrap_or_else(|| "A conflict has occurred".to_owned()),
        )
    }

    pub fn unprocessable_entity(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            code.unwrap_or_else(|| "validation".to_owned()),
            detail.unwrap_or_else(|| "Validation error".to_owned()),
        )
    }

    pub fn internal_server_errer(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            code.unwrap_or_else(|| "server_error".to_owned()),
            detail.unwrap_or_else(|| "Internal Server Error".to_owned()),
        )
    }

    pub fn not_implemented(code: Option<String>, detail: Option<String>) -> Self {
        Self::new(
            StatusCode::NOT_IMPLEMENTED,
            code.unwrap_or_else(|| "not_implemented".to_owned()),
            detail.unwrap_or_else(|| "This API endpoint is not yet implented.".to_owned()),
        )
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::Http(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "status={} code=\"{}\" detail=\"{}\"",
            self.status, self.body.code, self.body.detail
        )
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
