// this file is @generated
use std::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// The different classes of HTTP status codes:
///
/// - CodeNone = 0
/// - Code1xx = 100
/// - Code2xx = 200
/// - Code3xx = 300
/// - Code4xx = 400
/// - Code5xx = 500
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize_repr,
    Deserialize_repr,
)]
pub enum StatusCodeClass {
    #[default]
    CodeNone = 0,
    Code1xx = 100,
    Code2xx = 200,
    Code3xx = 300,
    Code4xx = 400,
    Code5xx = 500,
}

impl fmt::Display for StatusCodeClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i64)
    }
}
