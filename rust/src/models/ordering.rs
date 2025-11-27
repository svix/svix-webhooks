// this file is @generated
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

/// Defines the ordering in a listing of results.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Ordering {
    #[default]
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

impl fmt::Display for Ordering {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let value = match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        };
        f.write_str(value)
    }
}
