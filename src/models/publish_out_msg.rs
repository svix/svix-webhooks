// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishOutMsg {
    pub offset: u64,

    pub partition: u16,
}

impl PublishOutMsg {
    pub fn new(offset: u64, partition: u16) -> Self {
        Self { offset, partition }
    }
}
