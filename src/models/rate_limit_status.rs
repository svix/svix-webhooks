// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RateLimitStatus {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "block")]
    Block,
}

impl fmt::Display for RateLimitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Ok => "ok",
            Self::Block => "block",
        };
        f.write_str(value)
    }
}
