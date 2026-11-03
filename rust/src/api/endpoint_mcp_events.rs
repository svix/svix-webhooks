// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EndpointMcpEventsSubscribeOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointMcpEventsUnsubscribeOptions {
    pub idempotency_key: Option<String>,
}

pub struct EndpointMcpEvents<'a> {
    cfg: &'a Configuration,
}

impl<'a> EndpointMcpEvents<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or refresh an MCP Events subscription.
    pub async fn subscribe(
        &self,
        app_id: String,
        mcp_events_subscribe_in: McpEventsSubscribeIn,
        options: Option<EndpointMcpEventsSubscribeOptions>,
    ) -> Result<McpEventsSubscribeOut> {
        let EndpointMcpEventsSubscribeOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/app/{app_id}/mcp-events/subscribe",
        )
        .with_path_param("app_id", app_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(mcp_events_subscribe_in)
        .execute(self.cfg)
        .await
    }

    /// Stop an MCP Events subscription.
    ///
    /// Forward an MCP server's `events/unsubscribe` requests here. The
    /// subscription's endpoint is deleted. Unsubscribing from a subscription
    /// that doesn't exist succeeds, so this can be repeated.
    pub async fn unsubscribe(
        &self,
        app_id: String,
        mcp_events_unsubscribe_in: McpEventsUnsubscribeIn,
        options: Option<EndpointMcpEventsUnsubscribeOptions>,
    ) -> Result<()> {
        let EndpointMcpEventsUnsubscribeOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/app/{app_id}/mcp-events/unsubscribe",
        )
        .with_path_param("app_id", app_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(mcp_events_unsubscribe_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
