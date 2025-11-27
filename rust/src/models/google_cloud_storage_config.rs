// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

/// Configuration for a Google Cloud Storage sink.
///
/// Write stream events into the named bucket using the supplied Google Cloud
/// credentials.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GoogleCloudStorageConfig {
    pub bucket: String,

    /// Google Cloud Credentials JSON Object as a string.
    pub credentials: String,
}

impl GoogleCloudStorageConfig {
    pub fn new(
        bucket: String,
        credentials: String,
    ) -> Self {
        Self {
            bucket,
            credentials,
        }
    }
}
