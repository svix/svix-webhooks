// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_type_in::EventTypeIn, template_in::TemplateIn};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EnvironmentIn {
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeIn>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,

    #[serde(rename = "transformationTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation_templates: Option<Vec<TemplateIn>>,
}

impl EnvironmentIn {
    pub fn new() -> Self {
        Self {
            event_types: None,
            settings: None,
            transformation_templates: None,
        }
    }
}
