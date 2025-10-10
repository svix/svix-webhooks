// this file is @generated
use serde::{Deserialize, Serialize};

use super::connector_kind::ConnectorKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectorIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    pub logo: String,

    pub name: String,

    pub transformation: String,
}

impl ConnectorIn {
    pub fn new(logo: String, name: String, transformation: String) -> Self {
        Self {
            description: None,
            feature_flags: None,
            filter_types: None,
            instructions: None,
            kind: None,
            logo,
            name,
            transformation,
        }
    }
}
