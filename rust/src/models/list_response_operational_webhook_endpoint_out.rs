use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListResponseOperationalWebhookEndpointOut {
    pub data: Vec<super::OperationalWebhookEndpointOut>,
    pub done: bool,
    pub iterator: Option<String>,
    #[serde(rename = "prevIterator", skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseOperationalWebhookEndpointOut {
    pub fn new(
        data: Vec<super::OperationalWebhookEndpointOut>,
        done: bool,
        iterator: Option<String>,
    ) -> ListResponseOperationalWebhookEndpointOut {
        ListResponseOperationalWebhookEndpointOut {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
