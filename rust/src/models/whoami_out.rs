// this file is @generated
use serde::{Deserialize, Serialize};

use super::authentication_source::AuthenticationSource;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WhoamiOut {
    /// The environment ("organization") that the current token is attached to
    #[serde(rename = "envId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_id: Option<String>,

    /// The dispatch application that the current token is limited to, if there
    /// is one
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// The stream application that the current token is limited to, if there is
    /// one
    #[serde(rename = "streamAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_app_id: Option<String>,

    /// The source of the current request's authentication
    ///
    /// This field is unstable and may change in the future.
    #[serde(rename = "permissionSource")]
    pub permission_source: AuthenticationSource,

    /// The session associated with the current request
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}
