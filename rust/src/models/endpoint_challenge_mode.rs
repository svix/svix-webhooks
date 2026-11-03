// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// How an endpoint must respond to a challenge.
///
/// - `challenge-response`: the Standard Webhooks challenge-response check. The
///   endpoint must return `webhook-challenge-response`, calculated from the
///   challenge and its secret.
/// - `echo`: the endpoint must echo the challenge from the request body back as
///   `{"challenge": "..."}`. This doesn't prove that it holds the secret.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndpointChallengeMode {
    #[serde(rename = "challenge-response")]
    ChallengeResponse,
    #[serde(rename = "echo")]
    Echo,
}

impl fmt::Display for EndpointChallengeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::ChallengeResponse => "challenge-response",
            Self::Echo => "echo",
        };
        f.write_str(value)
    }
}
