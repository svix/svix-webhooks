use serde::{Deserialize, Serialize};

use super::ValidationError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpValidationError {
    pub detail: Vec<ValidationError>,
}

impl HttpValidationError {
    pub fn new(detail: Vec<ValidationError>) -> HttpValidationError {
        HttpValidationError { detail }
    }
}
