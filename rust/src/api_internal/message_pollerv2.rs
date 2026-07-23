// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MessagePollerv2ConsumerPollOptions {
    pub limit: Option<i32>,

    /// Lease duration in milliseconds.
    pub lease_duration_ms: Option<i32>,

    pub starting_position: Option<StartingPosition>,
}

#[derive(Default)]
pub struct MessagePollerv2ConsumerCommitOptions {
    pub idempotency_key: Option<String>,
}

pub struct MessagePollerv2<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessagePollerv2<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Poll messages from a sink.
    pub async fn consumer_poll(
        &self,
        app_id: String,
        sink_id: String,
        consumer_id: String,
        options: Option<MessagePollerv2ConsumerPollOptions>,
    ) -> Result<PollerV2PollOut> {
        let MessagePollerv2ConsumerPollOptions {
            limit,
            lease_duration_ms,
            starting_position,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("sink_id", sink_id)
        .with_path_param("consumer_id", consumer_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("lease_duration_ms", lease_duration_ms)
        .with_optional_query_param("starting_position", starting_position)
        .execute(self.cfg)
        .await
    }

    /// Ack a message offset for a sink's consumer.
    pub async fn consumer_commit(
        &self,
        app_id: String,
        sink_id: String,
        consumer_id: String,
        poller_v2_commit_in: PollerV2CommitIn,
        options: Option<MessagePollerv2ConsumerCommitOptions>,
    ) -> Result<()> {
        let MessagePollerv2ConsumerCommitOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http::Method::POST,
            "/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("sink_id", sink_id)
        .with_path_param("consumer_id", consumer_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(poller_v2_commit_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
