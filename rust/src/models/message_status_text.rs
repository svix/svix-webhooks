// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum MessageStatusText {
    #[default]
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "sending")]
    Sending,
}

impl fmt::Display for MessageStatusText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Success => "success",
            Self::Pending => "pending",
            Self::Fail => "fail",
            Self::Sending => "sending",
        };
        f.write_str(value)
    }
}
