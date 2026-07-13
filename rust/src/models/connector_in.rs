// this file is @generated
use serde::{Deserialize, Serialize};

use super::{connector_kind::ConnectorKind, connector_product::ConnectorProduct};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConnectorIn {
    pub name: String,

    /// The Connector's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(rename = "allowedEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_event_types: Option<Vec<String>>,

    pub transformation: String,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ConnectorProduct>,
}

impl ConnectorIn {
    pub fn new(name: String, transformation: String) -> Self {
        Self {
            name,
            uid: None,
            logo: None,
            description: None,
            kind: None,
            instructions: None,
            allowed_event_types: None,
            transformation,
            feature_flags: None,
            product_type: None,
        }
    }
}
