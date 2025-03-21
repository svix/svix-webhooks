use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EventsPublicConsumerOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filters messages sent with this event type (optional).
    pub event_type: Option<String>,

    /// Filters messages sent with this channel (optional).
    pub channel: Option<String>,
}

#[derive(Default)]
pub struct EventsPublicConsumerSeekOptions {
    pub idempotency_key: Option<String>,
}

pub struct EventsPublic<'a> {
    cfg: &'a Configuration,
}

impl<'a> EventsPublic<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Reads the stream of created messages for an application, filtered on the
    /// Sink's event types and Channels, using server-managed iterator
    /// tracking.
    pub async fn consumer(
        &self,
        app_id: String,
        sink_id: String,
        consumer_id: String,
        options: Option<EventsPublicConsumerOptions>,
    ) -> Result<PollingEndpointOut> {
        let EventsPublicConsumerOptions {
            limit,
            iterator,
            event_type,
            channel,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("sink_id", sink_id)
        .with_path_param("consumer_id", consumer_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("event_type", event_type)
        .with_optional_query_param("channel", channel)
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
        options: Option<EventsPublicConsumerSeekOptions>,
    ) -> Result<PollingEndpointConsumerSeekOut> {
        let EventsPublicConsumerSeekOptions { idempotency_key } = options.unwrap_or_default();

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
