// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ElasticSearchConfigOut {
    #[serde(rename = "indexName")]
    pub index_name: String,

    pub url: String,

    pub refresh: bool,
}
