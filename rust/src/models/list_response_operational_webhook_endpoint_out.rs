// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::operational_webhook_endpoint_out::OperationalWebhookEndpointOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseOperationalWebhookEndpointOut {
    pub data: Vec<OperationalWebhookEndpointOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseOperationalWebhookEndpointOut {
    pub fn new(
        data: Vec<OperationalWebhookEndpointOut>,
        done: bool,
    ) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
