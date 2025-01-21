use std::fmt;

use serde::{Deserialize, Serialize};

/// Defines the ordering in a listing of results.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize,
)]
pub enum Ordering {
    #[default]
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

impl fmt::Display for Ordering {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ascending => write!(f, "ascending"),
            Self::Descending => write!(f, "descending"),
        }
    }
}
