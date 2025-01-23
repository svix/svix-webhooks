// this file is @generated
use serde::{Deserialize, Serialize};

use super::transformation_template_kind::TransformationTemplateKind;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TemplateOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub description: String,

    #[serde(rename = "featureFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<String>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    pub id: String,

    pub instructions: String,

    #[serde(rename = "instructionsLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_link: Option<String>,

    pub kind: TransformationTemplateKind,

    pub logo: String,

    pub name: String,

    #[serde(rename = "orgId")]
    pub org_id: String,

    pub transformation: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl TemplateOut {
    pub fn new(
        created_at: String,
        description: String,
        id: String,
        instructions: String,
        kind: TransformationTemplateKind,
        logo: String,
        name: String,
        org_id: String,
        transformation: String,
        updated_at: String,
    ) -> Self {
        Self {
            created_at,
            description,
            feature_flag: None,
            filter_types: None,
            id,
            instructions,
            instructions_link: None,
            kind,
            logo,
            name,
            org_id,
            transformation,
            updated_at,
        }
    }
}
