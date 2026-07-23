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
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "transformationUpdatedAt")]
    pub transformation_updated_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<std::collections::BTreeSet<String>>,

    #[serde(rename = "productType")]
    pub product_type: ConnectorProduct,
}
