// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// The method used for authenticating to the OAuth authorization server.
///
/// `clientSecretJwt` will construct a JWT used for authentication
/// with the oauth authorization server. This method is less
/// commonly used and may not be supported by all oauth providers.
/// `clientSecretBasic` will authenticate to the oauth authorization server
/// using an `Authorization` header with the client secret as the value.
/// This is the most common means of authentication.
/// `clientSecretPost` will authenticate to the oauth authorization server
/// by passing the client secret in a `client_secret` field in the
/// request body. This method may not be supported by all oauth providers,
/// and in general `clientSecretBasic` should be preferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Oauth2AuthMethodInOut {
    #[serde(rename = "clientSecretJwt")]
    ClientSecretJwt,
    #[serde(rename = "clientSecretBasic")]
    ClientSecretBasic,
    #[serde(rename = "clientSecretPost")]
    ClientSecretPost,
}

impl fmt::Display for Oauth2AuthMethodInOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::ClientSecretJwt => "clientSecretJwt",
            Self::ClientSecretBasic => "clientSecretBasic",
            Self::ClientSecretPost => "clientSecretPost",
        };
        f.write_str(value)
    }
}
