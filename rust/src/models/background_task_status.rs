use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for BackgroundTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Finished => write!(f, "finished"),
            Self::Failed => write!(f, "failed"),
        }
    }
}
