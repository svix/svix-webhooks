// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Operation for bulk indexing.
///
/// `index` will do an upsert for the given document ID, and `create` will
/// insert it (and do nothing on conflict). Only `create` is supported for Data
/// Stream targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ElasticSearchOperation {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
}

impl fmt::Display for ElasticSearchOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Index => "index",
            Self::Create => "create",
        };
        f.write_str(value)
    }
}
