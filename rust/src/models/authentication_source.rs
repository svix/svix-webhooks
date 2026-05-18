// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// The authentication type of the current request
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum AuthenticationSource {
    #[default]
    #[serde(rename = "OidcJwt")]
    OidcJwt,
    #[serde(rename = "SvixJwt")]
    SvixJwt,
    #[serde(rename = "RegionalAuthToken")]
    RegionalAuthToken,
    #[serde(rename = "GlobalAuthToken")]
    GlobalAuthToken,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl fmt::Display for AuthenticationSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::OidcJwt => "OidcJwt",
            Self::SvixJwt => "SvixJwt",
            Self::RegionalAuthToken => "RegionalAuthToken",
            Self::GlobalAuthToken => "GlobalAuthToken",
            Self::Unknown => "Unknown",
        };
        f.write_str(value)
    }
}
