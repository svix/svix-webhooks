// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::connector_kind::ConnectorKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectorOut {
    #[serde(rename = "allowedEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_event_types: Option<Vec<String>>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub description: String,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// The Connector's ID.
    pub id: String,

    pub instructions: String,

    pub kind: ConnectorKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    pub name: String,

    /// The Environment's ID.
    #[serde(rename = "orgId")]
    pub org_id: String,

    pub transformation: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl ConnectorOut {
    pub fn new(
        created_at: String,
        description: String,
        id: String,
        instructions: String,
        kind: ConnectorKind,
        name: String,
        org_id: String,
        transformation: String,
        updated_at: String,
    ) -> Self {
        Self {
            allowed_event_types: None,
            created_at,
            description,
            feature_flags: None,
            id,
            instructions,
            kind,
            logo: None,
            name,
            org_id,
            transformation,
            updated_at,
        }
    }
}
