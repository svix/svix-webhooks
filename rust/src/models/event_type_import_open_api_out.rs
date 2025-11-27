// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::event_type_import_open_api_out_data::EventTypeImportOpenApiOutData;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeImportOpenApiOut {
    pub data: EventTypeImportOpenApiOutData,
}

impl EventTypeImportOpenApiOut {
    pub fn new(data: EventTypeImportOpenApiOutData) -> Self {
        Self {
            data,
        }
    }
}
