// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ElasticSearchConfigPatch {
    #[serde(rename = "indexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
}

impl ElasticSearchConfigPatch {
    pub fn new() -> Self {
        Self {
            index_name: None,
            url: None,
            api_key: None,
            refresh: None,
        }
    }
}

impl Default for ElasticSearchConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
