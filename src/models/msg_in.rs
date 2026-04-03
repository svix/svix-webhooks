// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgIn {
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// Optional partition key.
    ///
    /// Messages with the same key are routed to the same partition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Optional delay in milliseconds.
    ///
    /// The message will not be delivered to queue consumers
    /// until the delay has elapsed from the time of publish.
    #[serde(
        rename = "delay_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub delay: Option<std::time::Duration>,
}

impl MsgIn {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            headers: None,
            key: None,
            delay: None,
        }
    }

    pub fn with_headers(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.headers = value.into();
        self
    }

    pub fn with_key(mut self, value: impl Into<Option<String>>) -> Self {
        self.key = value.into();
        self
    }

    pub fn with_delay(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.delay = value.into();
        self
    }
}
