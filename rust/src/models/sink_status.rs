// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SinkStatus {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "retrying")]
    Retrying,
}

impl fmt::Display for SinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Enabled => "enabled",
            Self::Paused => "paused",
            Self::Disabled => "disabled",
            Self::Retrying => "retrying",
        };
        f.write_str(value)
    }
}
