// this file is @generated
use serde::{Deserialize, Serialize};

use super::connector_kind::ConnectorKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectorIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Deprecated - prefer featureFlags instead.
    #[deprecated]
    #[serde(rename = "featureFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(rename = "instructionsLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ConnectorKind>,

    pub logo: String,

    pub name: String,

    pub transformation: String,
}

impl ConnectorIn {
    pub fn new(logo: String, name: String, transformation: String) -> Self {
        #[allow(deprecated)]
        Self {
            description: None,
            feature_flag: None,
            feature_flags: None,
            filter_types: None,
            instructions: None,
            instructions_link: None,
            kind: None,
            logo,
            name,
            transformation,
        }
    }
}
