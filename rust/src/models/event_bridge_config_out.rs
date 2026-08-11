// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfigOut {
    #[serde(rename = "eventBusName")]
    pub event_bus_name: String,

    #[serde(rename = "detailType")]
    pub detail_type: String,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    pub region: String,
}
