// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    app_portal_capability::AppPortalCapability,
    application_in::ApplicationIn,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AppPortalAccessIn {
    /// Optionally creates a new application while generating the access link.
    ///
    /// If the application id or uid that is used in the path already exists,
    /// this argument is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationIn>,

    /// Custom capabilities attached to the token, You can combine as many
    /// capabilities as necessary.
    ///
    /// The `ViewBase` capability is always required
    ///
    /// - `ViewBase`: Basic read only permissions, does not allow the user to
    ///   see the endpoint secret.
    ///
    /// - `ViewEndpointSecret`: Allows user to view the endpoint secret.
    ///
    /// - `ManageEndpointSecret`: Allows user to rotate and view the endpoint
    ///   secret.
    ///
    /// - `ManageTransformations`: Allows user to modify the endpoint
    ///   transformations.
    ///
    /// - `CreateAttempts`: Allows user to replay missing messages and send
    ///   example messages.
    ///
    /// - `ManageEndpoint`: Allows user to read/modify any field or
    ///   configuration of an endpoint (including secrets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<AppPortalCapability>>,

    /// How long the token will be valid for, in seconds.
    ///
    /// Valid values are between 1 hour and 7 days. The default is 7 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,

    /// The set of feature flags the created token will have access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// Whether the app portal should be in read-only mode.
    #[deprecated]
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// An optional session ID to attach to the token.
    ///
    /// When expiring tokens with "Expire All", you can include the session ID
    /// to only expire tokens that were created with that session ID.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl AppPortalAccessIn {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            application: None,
            capabilities: None,
            expiry: None,
            feature_flags: None,
            read_only: None,
            session_id: None,
        }
    }
}
