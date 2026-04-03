// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MsgQueueReceiveIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    #[serde(
        rename = "lease_duration_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub lease_duration: Option<std::time::Duration>,

    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(
        rename = "batch_wait_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub batch_wait: Option<std::time::Duration>,
}

impl MsgQueueReceiveIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            batch_size: None,
            lease_duration: None,
            batch_wait: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_batch_size(mut self, value: impl Into<Option<u16>>) -> Self {
        self.batch_size = value.into();
        self
    }

    pub fn with_lease_duration(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.lease_duration = value.into();
        self
    }

    pub fn with_batch_wait(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.batch_wait = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueReceiveIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub consumer_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    #[serde(
        rename = "lease_duration_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub lease_duration: Option<std::time::Duration>,

    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(
        rename = "batch_wait_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub batch_wait: Option<std::time::Duration>,
}
