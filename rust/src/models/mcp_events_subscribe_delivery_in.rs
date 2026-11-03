// this file is @generated
use serde::{Deserialize, Serialize};

use super::mcp_events_delivery_mode::McpEventsDeliveryMode;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct McpEventsSubscribeDeliveryIn {
    /// How events are delivered. Only `webhook` is supported.
    pub mode: McpEventsDeliveryMode,

    /// The callback URL events are sent to.
    pub url: String,

    /// The secret events are signed with. MCP Events requires a `whsec_`
    /// secret.
    pub secret: String,
}

impl McpEventsSubscribeDeliveryIn {
    pub fn new(mode: McpEventsDeliveryMode, url: String, secret: String) -> Self {
        Self { mode, url, secret }
    }
}
