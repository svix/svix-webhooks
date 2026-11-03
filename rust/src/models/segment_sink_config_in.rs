// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a Segment sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SegmentSinkConfigIn {
    /// The Segment ingestion host.
    ///
    /// This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com
    /// for EU workspaces, or your custom domain. Events are sent to the
    /// `/v1/batch` endpoint under this host.
    pub host: String,

    /// The Segment source write key.
    ///
    /// Sent as HTTP Basic auth (the write key as the username, with an empty
    /// password) on each `/v1/batch` request.
    #[serde(rename = "writeKey")]
    pub write_key: String,
}

impl SegmentSinkConfigIn {
    pub fn new(host: String, write_key: String) -> Self {
        Self { host, write_key }
    }
}
