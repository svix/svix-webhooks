// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct StreamingEventsCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamingEventsGetOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    pub after: Option<String>,
}

pub struct StreamingEvents<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingEvents<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// Creates events on the Stream.
    pub async fn create(
        &self,
        stream_id: String,
        create_stream_events_in: CreateStreamEventsIn,
        options: Option<StreamingEventsCreateOptions>,
    ) -> Result<CreateStreamEventsOut> {
        let StreamingEventsCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/stream/{stream_id}/events",
        )
            .with_path_param("stream_id", stream_id)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(create_stream_events_in)
            .execute(self.cfg)
            .await
    }

    /// Iterate over a stream of events.
    ///
    /// The sink must be of type `poller` to use the poller endpoint.
    pub async fn get(
        &self,
        stream_id: String,
        sink_id: String,
        options: Option<StreamingEventsGetOptions>,
    ) -> Result<EventStreamOut> {
        let StreamingEventsGetOptions {
            limit,
            iterator,
            after,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/events",
        )
            .with_path_param("stream_id", stream_id)
            .with_path_param("sink_id", sink_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("after", after)
            .execute(self.cfg)
            .await
    }
}
