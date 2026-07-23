// this file is @generated
use serde::{Deserialize, Serialize};

use super::event_type_import_open_api_out_data::EventTypeImportOpenApiOutData;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypeImportOpenApiOut {
    pub data: EventTypeImportOpenApiOutData,
}
