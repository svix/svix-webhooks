// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::event_type_from_open_api::EventTypeFromOpenApi;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeImportOpenApiOutData {
    pub modified: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_modify: Option<Vec<EventTypeFromOpenApi>>,
}

impl EventTypeImportOpenApiOutData {
    pub fn new(modified: Vec<String>) -> Self {
        Self {
            modified,
            to_modify: None,
        }
    }
}
