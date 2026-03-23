// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgStreamReceiveIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_millis: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<String>,
}

impl MsgStreamReceiveIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            batch_size: None,
            lease_duration_millis: None,
            default_starting_position: None,
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

    pub fn with_lease_duration_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.lease_duration_millis = value.into();
        self
    }

    pub fn with_default_starting_position(mut self, value: impl Into<Option<String>>) -> Self {
        self.default_starting_position = value.into();
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_millis: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<String>,
}
