// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BulkExpungeStatus {
    #[serde(rename = "expunged")]
    Expunged,
    #[serde(rename = "not-found")]
    NotFound,
}

impl fmt::Display for BulkExpungeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Expunged => "expunged",
            Self::NotFound => "not-found",
        };
        f.write_str(value)
    }
}
