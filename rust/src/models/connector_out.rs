// this file is @generated
use serde::{Deserialize, Serialize};

use super::{connector_kind::ConnectorKind, connector_product::ConnectorProduct};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConnectorOut {
    /// The Connector's ID.
    pub id: String,

    /// The Environment's ID.
    #[serde(rename = "orgId")]
    pub org_id: String,

    /// The Connector's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    pub kind: ConnectorKind,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    pub description: String,

    pub instructions: String,

    #[serde(rename = "allowedEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_event_types: Option<std::collections::BTreeSet<String>>,

    pub transformation: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "transformationUpdatedAt")]
    pub transformation_updated_at: String,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<std::collections::BTreeSet<String>>,

    #[serde(rename = "productType")]
    pub product_type: ConnectorProduct,
}

impl ConnectorOut {
    pub fn new(
        id: String,
        org_id: String,
        kind: ConnectorKind,
        name: String,
        description: String,
        instructions: String,
        transformation: String,
        created_at: String,
        updated_at: String,
        transformation_updated_at: String,
        product_type: ConnectorProduct,
    ) -> Self {
        Self {
            id,
            org_id,
            uid: None,
            kind,
            name,
            logo: None,
            description,
            instructions,
            allowed_event_types: None,
            transformation,
            created_at,
            updated_at,
            transformation_updated_at,
            feature_flags: None,
            product_type,
        }
    }
}
