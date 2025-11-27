use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpValidationError {
    #[serde(rename = "detail")]
    pub detail: Vec<super::ValidationError>,
}

impl HttpValidationError {
    pub fn new(detail: Vec<super::ValidationError>) -> HttpValidationError {
        HttpValidationError {
            detail,
        }
    }
}
