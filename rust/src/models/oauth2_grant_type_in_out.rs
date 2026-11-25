// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Oauth2GrantTypeInOut {
    #[serde(rename = "clientCredentials")]
    ClientCredentials,
    #[serde(rename = "refreshToken")]
    RefreshToken,
}

impl fmt::Display for Oauth2GrantTypeInOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::ClientCredentials => "clientCredentials",
            Self::RefreshToken => "refreshToken",
        };
        f.write_str(value)
    }
}
