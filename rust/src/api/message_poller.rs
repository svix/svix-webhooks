// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MessagePollerPollOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filters messages sent with this event type (optional).
    pub event_type: Option<String>,

    /// Filters messages sent with this channel (optional).
    pub channel: Option<String>,

    pub after: Option<String>,
}

#[derive(Default)]
pub struct MessagePollerConsumerPollOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
}

#[derive(Default)]
pub struct MessagePollerConsumerSeekOptions {
    pub idempotency_key: Option<String>,
}

pub struct MessagePoller<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessagePoller<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Reads the stream of created messages for an application, filtered on the
    /// Sink's event types and Channels.
    pub async fn poll(
        &self,
        app_id: String,
        sink_id: String,
        options: Option<MessagePollerPollOptions>,
    ) -> Result<PollingEndpointOut> {
        let MessagePollerPollOptions {
            limit,
            iterator,
            event_type,
            channel,
            after,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/poller/{sink_id}")
            .with_path_param("app_id", app_id)
            .with_path_param("sink_id", sink_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("event_type", event_type)
            .with_optional_query_param("channel", channel)
            .with_optional_query_param("after", after)
            .execute(self.cfg)
            .await
    }

    /// Reads the stream of created messages for an application, filtered on the
    /// Sink's event types and Channels, using server-managed iterator
    /// tracking.
    pub async fn consumer_poll(
        &self,
        app_id: String,
        sink_id: String,
        consumer_id: String,
        options: Option<MessagePollerConsumerPollOptions>,
    ) -> Result<PollingEndpointOut> {
        let MessagePollerConsumerPollOptions { limit, iterator } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("sink_id", sink_id)
        .with_path_param("consumer_id", consumer_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .execute(self.cfg)
        .await
    }

    /// Sets the starting offset for the consumer of a polling endpoint.
    pub async fn consumer_seek(
        &self,
        app_id: String,
        sink_id: String,
        consumer_id: String,
        polling_endpoint_consumer_seek_in: PollingEndpointConsumerSeekIn,
        options: Option<MessagePollerConsumerSeekOptions>,
    ) -> Result<PollingEndpointConsumerSeekOut> {
        let MessagePollerConsumerSeekOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("sink_id", sink_id)
        .with_path_param("consumer_id", consumer_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(polling_endpoint_consumer_seek_in)
        .execute(self.cfg)
        .await
    }
}
