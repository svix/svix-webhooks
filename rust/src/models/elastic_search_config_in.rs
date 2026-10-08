// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining an ElasticSearch/OpenSearch sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ElasticSearchConfigIn {
    /// Name of the index to write to
    ///
    /// This can also be a data stream on sufficiently-new versions of
    /// ElasticSearch/OpenSearch
    #[serde(rename = "indexName")]
    pub index_name: String,

    /// Base URL to send indexing requests
    ///
    /// This should not include the /{index}/_bulk suffix.
    pub url: String,

    /// API key for authentication for ElasticSearch
    ///
    /// If not passed, any username:password embedded in the URL will be used.
    /// If none is passed, the indexing will be done unauthenticated.
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// If true, Elasticsearch refreshes the affected shards to make this
    /// operation visible to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
}

impl ElasticSearchConfigIn {
    pub fn new(index_name: String, url: String) -> Self {
        Self {
            index_name,
            url,
            api_key: None,
            refresh: None,
        }
    }
}
