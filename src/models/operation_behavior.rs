// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum OperationBehavior {
    #[serde(rename = "upsert")]
    Upsert,
    #[serde(rename = "insert")]
    Insert,
    #[serde(rename = "update")]
    Update,
}

impl fmt::Display for OperationBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Upsert => "upsert",
            Self::Insert => "insert",
            Self::Update => "update",
        };
        f.write_str(value)
    }
}
