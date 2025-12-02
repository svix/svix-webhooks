// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct HttpAttemptTimes {
    pub end: String,

    pub start: String,
}

impl HttpAttemptTimes {
    pub fn new(end: String, start: String) -> Self {
        Self { end, start }
    }
}
