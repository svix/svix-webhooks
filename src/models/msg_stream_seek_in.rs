// this file is @generated
use serde::{Deserialize, Serialize};

use super::seek_position::SeekPosition;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MsgStreamSeekIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<SeekPosition>,
}

impl MsgStreamSeekIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            offset: None,
            position: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_offset(mut self, value: impl Into<Option<u64>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn with_position(mut self, value: impl Into<Option<SeekPosition>>) -> Self {
        self.position = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgStreamSeekIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    pub consumer_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<SeekPosition>,
}
