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

/// A short-hand version of a [std::result::Result] that always returns an Svix [Error].
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
            Error::Generic(s) => s.fmt(f),
            Error::Database(s) => s.fmt(f),
            Error::Queue(s) => s.fmt(f),
            Error::Validation(s) => s.fmt(f),
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

impl<T: error::Error + 'static> From<bb8::RunError<T>> for Error {
    fn from(err: bb8::RunError<T>) -> Self {
        Error::Queue(err.to_string())
    }
}
impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Queue(err.to_string())
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
        match self {
            Error::Http(s) => {
                tracing::debug!("{:?}", &s);
                s.into_response()
            }
            s => {
                tracing::error!("{:?}", &s);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpErrorBody {
    Standard { code: String, detail: String },
    Validation { detail: Vec<ValidationErrorItem> },
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
/// Validation errors have their own schema to provide context for invalid requests eg. mismatched
/// types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY
/// error.
pub struct ValidationErrorItem {
    /// The location as a [`Vec`] of [`String`]s -- often in the form `["body", "field_name"]`,
    /// `["query", "field_name"]`, etc. They may, however, be arbitarily deep.
    pub loc: Vec<String>,

    /// The message accompanying the validation error item.
    pub msg: String,

    /// The type of error, often "type_error" or "value_error", but sometimes with more context like
    /// as "value_error.number.not_ge"
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct HttpError {
    status: StatusCode,
    body: HttpErrorBody,
}

impl HttpError {
    fn new_standard(status: StatusCode, code: String, detail: String) -> Self {
        Self {
            status,
            body: HttpErrorBody::Standard { code, detail },
        }
    }

    pub fn bad_request(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::BAD_REQUEST,
            code.unwrap_or_else(|| "generic_error".to_owned()),
            detail.unwrap_or_else(|| "Generic error".to_owned()),
        )
    }

    pub fn not_found(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::NOT_FOUND,
            code.unwrap_or_else(|| "not_found".to_owned()),
            detail.unwrap_or_else(|| "Entity not fonud".to_owned()),
        )
    }

    pub fn unauthorized(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::UNAUTHORIZED,
            code.unwrap_or_else(|| "authentication_failed".to_owned()),
            detail.unwrap_or_else(|| "Incorrect authentication credentials.".to_owned()),
        )
    }

    pub fn permission_denied(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::FORBIDDEN,
            code.unwrap_or_else(|| "insufficient access".to_owned()),
            detail.unwrap_or_else(|| "Insufficient access for the given operation.".to_owned()),
        )
    }

    pub fn conflict(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::CONFLICT,
            code.unwrap_or_else(|| "conflict".to_owned()),
            detail.unwrap_or_else(|| "A conflict has occurred".to_owned()),
        )
    }

    pub fn unprocessable_entity(detail: Vec<ValidationErrorItem>) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            body: HttpErrorBody::Validation { detail },
        }
    }

    pub fn internal_server_errer(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::INTERNAL_SERVER_ERROR,
            code.unwrap_or_else(|| "server_error".to_owned()),
            detail.unwrap_or_else(|| "Internal Server Error".to_owned()),
        )
    }

    pub fn not_implemented(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
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
        match &self.body {
            HttpErrorBody::Standard { code, detail } => write!(
                f,
                "status={} code=\"{}\" detail=\"{}\"",
                self.status, code, detail
            ),

            HttpErrorBody::Validation { detail } => {
                write!(
                    f,
                    "status={} detail={}",
                    self.status,
                    serde_json::to_string(&detail)
                        .unwrap_or_else(|e| format!("\"unserializable error for {}\"", e))
                )
            }
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
