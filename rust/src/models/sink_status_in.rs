// this file is @generated
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum SinkStatusIn {
    #[default]
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl fmt::Display for SinkStatusIn {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let value = match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        };
        f.write_str(value)
    }
}
