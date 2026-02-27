// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "Persistent")]
    Persistent,
    #[serde(rename = "Ephemeral")]
    Ephemeral,
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Persistent => "Persistent",
            Self::Ephemeral => "Ephemeral",
        };
        f.write_str(value)
    }
}
