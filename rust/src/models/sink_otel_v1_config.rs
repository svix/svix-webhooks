// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SinkOtelV1Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub url: String,
}

impl SinkOtelV1Config {
    pub fn new(url: String) -> Self {
        Self {
            headers: None,
            url,
        }
    }
}
