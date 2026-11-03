// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    endpoint_challenge_mode::EndpointChallengeMode,
    mcp_events_subscribe_delivery_in::McpEventsSubscribeDeliveryIn,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct McpEventsSubscribeIn {
    /// The event to subscribe to. It must be an existing event type.
    pub name: String,

    /// The filter arguments of the `events/subscribe` request, as described by
    /// the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
    /// Together with `name` and the callback URL they identify the subscription
    ///
    /// They're stored in the endpoint's `mcpEventArguments` metadata, but
    /// aren't used to filter messages; use `channels` for that.
    ///
    /// See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
    /// in OpenAI's MCP Events docs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,

    pub delivery: McpEventsSubscribeDeliveryIn,

    /// The requested lifetime of the subscription, in milliseconds. Omit for
    /// the default of 7 days, or set to `null` for a subscription that never
    /// expires.
    #[serde(rename = "ttlMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,

    /// How long the previous secret keeps signing when a refresh changes it,
    /// in seconds.
    ///
    /// Valid values are between 0 (immediate expiry) and 7 days. The default
    /// is 5 minutes.
    #[serde(rename = "secretGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_grace_period_seconds: Option<u32>,

    /// How the callback must respond to the verification challenge.
    #[serde(rename = "challengeMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_mode: Option<EndpointChallengeMode>,

    /// Only send messages with one of these channels to the subscription,
    /// for example ones derived from `arguments`. Omit to send all messages
    /// of the event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<std::collections::BTreeSet<String>>,
}

impl McpEventsSubscribeIn {
    pub fn new(name: String, delivery: McpEventsSubscribeDeliveryIn) -> Self {
        Self {
            name,
            arguments: None,
            delivery,
            ttl_ms: None,
            secret_grace_period_seconds: None,
            challenge_mode: None,
            channels: None,
        }
    }
}
