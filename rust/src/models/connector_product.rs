// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum ConnectorProduct {
    #[default]
    #[serde(rename = "Dispatch")]
    Dispatch,
    #[serde(rename = "Stream")]
    Stream,
}

impl fmt::Display for ConnectorProduct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Dispatch => "Dispatch",
            Self::Stream => "Stream",
        };
        f.write_str(value)
    }
}
