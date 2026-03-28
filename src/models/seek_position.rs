// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SeekPosition {
    #[serde(rename = "earliest")]
    Earliest,
    #[serde(rename = "latest")]
    Latest,
}

impl fmt::Display for SeekPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Earliest => "earliest",
            Self::Latest => "latest",
        };
        f.write_str(value)
    }
}
