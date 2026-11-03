// this file is @generated
use serde::{Deserialize, Serialize};

use super::mcp_events_delivery_mode::McpEventsDeliveryMode;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct McpEventsUnsubscribeDeliveryIn {
    /// How events are delivered. Only `webhook` is supported.
    pub mode: McpEventsDeliveryMode,

    /// The callback URL the subscription was made with.
    pub url: String,
}

impl McpEventsUnsubscribeDeliveryIn {
    pub fn new(mode: McpEventsDeliveryMode, url: String) -> Self {
        Self { mode, url }
    }
}
