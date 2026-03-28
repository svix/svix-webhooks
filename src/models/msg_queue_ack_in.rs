// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgQueueAckIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub msg_ids: Vec<String>,
}

impl MsgQueueAckIn {
    pub fn new(msg_ids: Vec<String>) -> Self {
        Self {
            namespace: None,
            msg_ids,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueAckIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub consumer_group: String,

    pub msg_ids: Vec<String>,
}
