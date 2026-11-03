// this file is @generated
use serde::{Deserialize, Serialize};

use super::mcp_events_unsubscribe_delivery_in::McpEventsUnsubscribeDeliveryIn;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct McpEventsUnsubscribeIn {
    /// The event the subscription was made for.
    pub name: String,

    /// The filter arguments the subscription was made with, as given to
    /// `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
    /// in OpenAI's MCP Events docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,

    pub delivery: McpEventsUnsubscribeDeliveryIn,
}

impl McpEventsUnsubscribeIn {
    pub fn new(name: String, delivery: McpEventsUnsubscribeDeliveryIn) -> Self {
        Self {
            name,
            arguments: None,
            delivery,
        }
    }
}
