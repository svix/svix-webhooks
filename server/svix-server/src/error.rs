// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
use axum::{
    extract::rejection::{ExtensionRejection, PathRejection},
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use schemars::JsonSchema;
use sea_orm::{DbErr, RuntimeErr, TransactionError};
use serde::Serialize;
use serde_json::json;

use crate::core::webhook_http_client;

/// A short-hand version of a [`std::result::Result`] that defaults to Svix'es [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned from the Svix API
#[derive(Debug)]
pub struct Error {
    // the file name and line number of the error. Used for debugging non Http errors
    pub trace: Vec<&'static Location<'static>>,
    pub typ: ErrorType,
}

impl Error {
    #[track_caller]
    pub fn generic(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Generic(s.to_string()),
        }
    }

    #[track_caller]
    pub fn database(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Database(s.to_string()),
        }
    }

    #[track_caller]
    pub fn queue(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Queue(s.to_string()),
        }
    }

    #[track_caller]
    pub fn validation(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Validation(s.to_string()),
        }
    }

    #[track_caller]
    pub fn http(h: HttpError) -> Self {
        Self {
            trace: Vec::with_capacity(0), // no debugging necessary
            typ: ErrorType::Http(h),
        }
    }

    #[track_caller]
    pub fn cache(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Cache(s.to_string()),
        }
    }

    #[track_caller]
    pub fn timeout(s: impl fmt::Display) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Timeout(s.to_string()),
        }
    }

    #[track_caller]
    pub fn trace(mut self) -> Self {
        self.trace.push(Location::caller());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.typ.fmt(f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let stringified: Vec<String> = self.trace.into_iter().map(ToString::to_string).collect();
        match self.typ {
            ErrorType::Http(s) => {
                tracing::debug!("{:?}, location: {:?}", &s, stringified);
                s.into_response()
            }
            s => {
                tracing::error!("type: {:?}, location: {:?}", s, stringified);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response()
            }
        }
    }
}

impl OperationOutput for Error {
    type Inner = Self;
}

pub trait Traceable<T> {
    /// Pushes the current [`Location`] onto the error's trace stack
    #[track_caller]
    fn trace(self) -> Result<T>;
}

impl<T> Traceable<T> for Result<T> {
    fn trace(self) -> Result<T> {
        // Using `map_err` would lose `#[track_caller]` information
        match self {
            Err(e) => Err(e.trace()),
            ok => ok,
        }
    }
}

impl From<DbErr> for Error {
    #[track_caller]
    fn from(value: DbErr) -> Self {
        Error::database(value)
    }
}

impl From<redis::RedisError> for Error {
    #[track_caller]
    fn from(value: redis::RedisError) -> Self {
        Error::queue(value)
    }
}

impl From<omniqueue::QueueError> for Error {
    #[track_caller]
    fn from(value: omniqueue::QueueError) -> Self {
        Error::queue(value)
    }
}

impl<E: error::Error + 'static> From<bb8::RunError<E>> for Error {
    #[track_caller]
    fn from(value: bb8::RunError<E>) -> Self {
        Error::queue(value)
    }
}

impl From<ExtensionRejection> for Error {
    #[track_caller]
    fn from(value: ExtensionRejection) -> Self {
        Error::generic(value)
    }
}

impl From<PathRejection> for Error {
    #[track_caller]
    fn from(value: PathRejection) -> Self {
        Error::generic(value)
    }
}

impl From<crate::core::cache::Error> for Error {
    #[track_caller]
    fn from(value: crate::core::cache::Error) -> Self {
        Error::cache(value)
    }
}

impl From<TransactionError<Error>> for Error {
    #[track_caller]
    fn from(value: TransactionError<Error>) -> Self {
        match value {
            TransactionError::Connection(db_err) => Error::database(db_err),
            TransactionError::Transaction(crate_err) => crate_err, // preserve the trace that comes from within the transaction
        }
    }
}

impl From<lapin::Error> for Error {
    #[track_caller]
    fn from(value: lapin::Error) -> Self {
        Error::queue(format!("{value:?}"))
    }
}

#[derive(Debug, Clone)]
pub enum ErrorType {
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
    /// Cache error
    Cache(String),
    /// Timeout error
    Timeout(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) => s.fmt(f),
            Self::Database(s) => s.fmt(f),
            Self::Queue(s) => s.fmt(f),
            Self::Validation(s) => s.fmt(f),
            Self::Http(s) => s.fmt(f),
            Self::Cache(s) => s.fmt(f),
            Self::Timeout(s) => s.fmt(f),
        }
    }
}

impl From<HttpError> for ErrorType {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}

// Python generation relies on the title of this being `HttpError`
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[schemars(rename = "HttpErrorOut", title = "HttpError")]
pub struct StandardHttpError {
    code: String,
    detail: String,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[schemars(rename = "HTTPValidationError")]
pub struct ValidationHttpError {
    detail: Vec<ValidationErrorItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpErrorBody {
    Standard(StandardHttpError),
    Validation(ValidationHttpError),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
/// Validation errors have their own schema to provide context for invalid requests eg. mismatched
/// types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY
/// error.
pub struct ValidationErrorItem {
    /// The location as a [`Vec`] of [`String`]s -- often in the form `["body", "field_name"]`,
    /// `["query", "field_name"]`, etc. They may, however, be arbitrarily deep.
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
    pub status: StatusCode,
    body: HttpErrorBody,
}

impl HttpError {
    fn new_standard(status: StatusCode, code: String, detail: String) -> Self {
        Self {
            status,
            body: HttpErrorBody::Standard(StandardHttpError { code, detail }),
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
            detail.unwrap_or_else(|| "Entity not found".to_owned()),
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
            body: HttpErrorBody::Validation(ValidationHttpError { detail }),
        }
    }

    pub fn internal_server_error(code: Option<String>, detail: Option<String>) -> Self {
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
            detail.unwrap_or_else(|| "This API endpoint is not yet implemented.".to_owned()),
        )
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::http(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.body {
            HttpErrorBody::Standard(StandardHttpError { code, detail }) => write!(
                f,
                "status={} code=\"{code}\" detail=\"{detail}\"",
                self.status
            ),

            HttpErrorBody::Validation(ValidationHttpError { detail }) => {
                write!(
                    f,
                    "status={} detail={}",
                    self.status,
                    serde_json::to_string(&detail)
                        .unwrap_or_else(|e| format!("\"unserializable error for {e}\""))
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

impl From<ErrorType> for Error {
    fn from(typ: ErrorType) -> Self {
        Self { trace: vec![], typ }
    }
}

// FIXME - delete
impl From<crate::core::webhook_http_client::Error> for Error {
    fn from(err: webhook_http_client::Error) -> Error {
        match err {
            webhook_http_client::Error::TimedOut => Self::timeout(err),
            _ => Error::generic(err),
        }
    }
}

/// Utility function for Converting a [`DbErr`] into an [`HttpError`] (wrapped by [`Error`]) on the
/// error "duplicate key value violates unique constraint". This is to be used in `map_err` calls
/// on creation/update of records
pub fn http_error_on_conflict(db_err: DbErr) -> Error {
    match db_err {
        DbErr::Query(RuntimeErr::SqlxError(e))
            if e.as_database_error()
                .and_then(|e| e.code())
                .filter(|code| code == "23505")
                .is_some() =>
        {
            HttpError::conflict(None, None).into()
        }
        e => Error::database(e),
    }
}
