// this file is @generated
use serde::{Deserialize, Serialize};

use super::{connector_kind::ConnectorKind, connector_product::ConnectorProduct};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectorIn {
    #[serde(rename = "allowedEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_event_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    pub name: String,

    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ConnectorProduct>,

    pub transformation: String,

    /// The Connector's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl ConnectorIn {
    pub fn new(name: String, transformation: String) -> Self {
        Self {
            allowed_event_types: None,
            description: None,
            feature_flags: None,
            instructions: None,
            kind: None,
            logo: None,
            name,
            product_type: None,
            transformation,
            uid: None,
        }
    }
}
