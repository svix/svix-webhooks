// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct StreamingEventTypeListOptions {
    /// Limit the number of returned items
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,

    /// Include archived items in the response.
    ///
    /// Deprecated, use `archive_status` instead. Ignored when `archive_status`
    /// is set.
    pub include_archived: Option<bool>,

    /// Which items to include based on whether they are archived.
    ///
    /// Defaults to `active`.
    pub archive_status: Option<ArchiveStatus>,
}

#[derive(Default)]
pub struct StreamingEventTypeCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamingEventTypeDeleteOptions {
    /// Entirely delete the event type, rather than archiving it.
    pub expunge: Option<bool>,
}

pub struct StreamingEventType<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingEventType<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List of all the organization's event types for streaming.
    pub async fn list(
        &self,
        options: Option<StreamingEventTypeListOptions>,
    ) -> Result<ListResponseStreamEventTypeOut> {
        let StreamingEventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
            archive_status,
        } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/stream/event-type")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("archive_status", archive_status)
            .execute(self.cfg)
            .await
    }

    /// Create an event type for Streams.
    pub async fn create(
        &self,
        stream_event_type_in: StreamEventTypeIn,
        options: Option<StreamingEventTypeCreateOptions>,
    ) -> Result<StreamEventTypeOut> {
        let StreamingEventTypeCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/event-type")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(stream_event_type_in)
            .execute(self.cfg)
            .await
    }

    /// Get an event type.
    pub async fn get(&self, name: String) -> Result<StreamEventTypeOut> {
        crate::request::Request::new(http::Method::GET, "/api/v1/stream/event-type/{name}")
            .with_path_param("name", name)
            .execute(self.cfg)
            .await
    }

    /// Create or update or create a event type for Streams.
    pub async fn upsert(
        &self,
        name: String,
        stream_event_type_in: StreamEventTypeIn,
    ) -> Result<StreamEventTypeOut> {
        crate::request::Request::new(http::Method::PUT, "/api/v1/stream/event-type/{name}")
            .with_path_param("name", name)
            .with_body_param(stream_event_type_in)
            .execute(self.cfg)
            .await
    }

    /// Archive an event type.
    ///
    /// An event type can be unarchived with the
    /// [patch operation](#operation/v1.streaming.event-type.patch) by setting
    /// `archived` to `false`.
    pub async fn delete(
        &self,
        name: String,
        options: Option<StreamingEventTypeDeleteOptions>,
    ) -> Result<()> {
        let StreamingEventTypeDeleteOptions { expunge } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::DELETE, "/api/v1/stream/event-type/{name}")
            .with_path_param("name", name)
            .with_optional_query_param("expunge", expunge)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Patch an event type for Streams.
    pub async fn patch(
        &self,
        name: String,
        stream_event_type_patch: StreamEventTypePatch,
    ) -> Result<StreamEventTypeOut> {
        crate::request::Request::new(http::Method::PATCH, "/api/v1/stream/event-type/{name}")
            .with_path_param("name", name)
            .with_body_param(stream_event_type_patch)
            .execute(self.cfg)
            .await
    }
}
