// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgQueueConfigureIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_schedule: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_topic: Option<String>,
}

impl MsgQueueConfigureIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            retry_schedule: None,
            dlq_topic: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_retry_schedule(mut self, value: impl Into<Option<Vec<u64>>>) -> Self {
        self.retry_schedule = value.into();
        self
    }

    pub fn with_dlq_topic(mut self, value: impl Into<Option<String>>) -> Self {
        self.dlq_topic = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueConfigureIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub consumer_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_schedule: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_topic: Option<String>,
}
