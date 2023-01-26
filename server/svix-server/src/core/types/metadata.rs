use std::collections::HashMap;

use crate::json_wrapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const MAX_METADATA_SIZE: usize = 4096;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default, JsonSchema)]
pub struct Metadata(HashMap<String, String>);

json_wrapper!(Metadata);

impl Metadata {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de> Deserialize<'de> for Metadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner: Option<HashMap<String, String>> = Deserialize::deserialize(deserializer)?;
        let metadata = inner.unwrap_or_default(); // coerce `null` to `{}`

        let size = serde_json::to_string(&metadata)
            .map(|blob| blob.len())
            .map_err(|_| serde::de::Error::custom("metadata is not valid json"))?;

        if size > MAX_METADATA_SIZE {
            return Err(serde::de::Error::custom(format!(
                "metadata must be less than or equal to {MAX_METADATA_SIZE} bytes"
            )));
        }

        Ok(Self(metadata))
    }
}
