// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Consistency level for reads.
///
/// Strong consistency (also known as linearizability) guarantees that a read will see all previous
/// writes. Weak consistency allows stale reads, but can save one or more round trip to the leader.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Consistency {
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "weak")]
    Weak,
}

impl fmt::Display for Consistency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Strong => "strong",
            Self::Weak => "weak",
        };
        f.write_str(value)
    }
}
