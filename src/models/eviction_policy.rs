// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EvictionPolicy {
    #[serde(rename = "NoEviction")]
    NoEviction,
    #[serde(rename = "LeastRecentlyUsed")]
    LeastRecentlyUsed,
}

impl fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::NoEviction => "NoEviction",
            Self::LeastRecentlyUsed => "LeastRecentlyUsed",
        };
        f.write_str(value)
    }
}
