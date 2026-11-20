// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OauthJwsSigningAlgorithm {
    #[serde(rename = "RS256")]
    Rs256,
}

impl fmt::Display for OauthJwsSigningAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Rs256 => "RS256",
        };
        f.write_str(value)
    }
}
