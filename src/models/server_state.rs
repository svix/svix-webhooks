// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ServerState {
    #[serde(rename = "Leader")]
    Leader,
    #[serde(rename = "Follower")]
    Follower,
    #[serde(rename = "Learner")]
    Learner,
    #[serde(rename = "Candidate")]
    Candidate,
    #[serde(rename = "Shutdown")]
    Shutdown,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl fmt::Display for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Leader => "Leader",
            Self::Follower => "Follower",
            Self::Learner => "Learner",
            Self::Candidate => "Candidate",
            Self::Shutdown => "Shutdown",
            Self::Unknown => "Unknown",
        };
        f.write_str(value)
    }
}
