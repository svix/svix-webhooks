// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_type_out::EventTypeOut, template_out::TemplateOut};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EnvironmentOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "eventTypes")]
    pub event_types: Vec<EventTypeOut>,

    pub settings: serde_json::Value,

    #[serde(rename = "transformationTemplates")]
    pub transformation_templates: Vec<TemplateOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl EnvironmentOut {
    pub fn new(
        created_at: String,
        event_types: Vec<EventTypeOut>,
        settings: serde_json::Value,
        transformation_templates: Vec<TemplateOut>,
    ) -> Self {
        Self {
            created_at,
            event_types,
            settings,
            transformation_templates,
            version: None,
        }
    }
}
