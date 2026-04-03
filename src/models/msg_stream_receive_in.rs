// this file is @generated
use serde::{Deserialize, Serialize};

use super::seek_position::SeekPosition;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MsgStreamReceiveIn {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<SeekPosition>,

    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(
        rename = "batch_wait_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub batch_wait: Option<std::time::Duration>,
}

impl MsgStreamReceiveIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            batch_size: None,
            lease_duration: None,
            default_starting_position: None,
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

    pub fn with_default_starting_position(
        mut self,
        value: impl Into<Option<SeekPosition>>,
    ) -> Self {
        self.default_starting_position = value.into();
        self
    }

    pub fn with_batch_wait(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.batch_wait = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgStreamReceiveIn_ {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<SeekPosition>,

    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(
        rename = "batch_wait_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub batch_wait: Option<std::time::Duration>,
}
