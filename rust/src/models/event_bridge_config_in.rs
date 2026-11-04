// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfigIn {
    /// The name or ARN of the event bus to receive the event
    #[serde(rename = "eventBusName")]
    pub event_bus_name: String,

    /// Free-form string, with a maximum of 128 characters
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    /// Access key ID.
    ///
    /// Required (along with `secret_access_key`) if `role_arn` is blank.
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Required (along with `access_key_id`) if `role_arn` is blank.
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    /// Role ARN for delegated authentication
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,

    /// Shared secret passed as the STS ExternalId.
    ///
    /// Can only be set if `role_arn` is Some
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// The region of the EventBridge bus.
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl EventBridgeConfigIn {
    pub fn new(event_bus_name: String) -> Self {
        Self {
            event_bus_name,
            detail_type: None,
            access_key_id: None,
            secret_access_key: None,
            role_arn: None,
            external_id: None,
            region: None,
        }
    }
}
