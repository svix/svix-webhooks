// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FetchFromStreamIn {
    /// How many messages to read from the stream.
    pub batch_size: u16,

    pub consumer_group: String,

    pub name: String,

    /// How long messages are locked by the consumer.
    pub visibility_timeout_seconds: u64,
}

impl FetchFromStreamIn {
    pub fn new(
        batch_size: u16,
        consumer_group: String,
        name: String,
        visibility_timeout_seconds: u64,
    ) -> Self {
        Self {
            batch_size,
            consumer_group,
            name,
            visibility_timeout_seconds,
        }
    }
}
