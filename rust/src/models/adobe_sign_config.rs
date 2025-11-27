// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AdobeSignConfig {
    #[serde(rename = "clientId")]
    pub client_id: String,
}

impl AdobeSignConfig {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
        }
    }
}
