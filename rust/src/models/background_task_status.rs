// this file is @generated
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BackgroundTaskStatus {
    #[default]
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "failed")]
    Failed,
}

impl fmt::Display for BackgroundTaskStatus {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let value = match self {
            Self::Running => "running",
            Self::Finished => "finished",
            Self::Failed => "failed",
        };
        f.write_str(value)
    }
}
