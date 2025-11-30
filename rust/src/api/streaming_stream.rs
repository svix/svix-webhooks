// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct StreamingStreamListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct StreamingStreamCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct StreamingStream<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingStream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List of all the organization's streams.
    pub async fn list(
        &self,
        options: Option<StreamingStreamListOptions>,
    ) -> Result<ListResponseStreamOut> {
        let StreamingStreamListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream",
        )
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Creates a new stream.
    pub async fn create(
        &self,
        stream_in: StreamIn,
        options: Option<StreamingStreamCreateOptions>,
    ) -> Result<StreamOut> {
        let StreamingStreamCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/stream",
        )
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(stream_in)
            .execute(self.cfg)
            .await
    }

    /// Get a stream by id or uid.
    pub async fn get(
        &self,
        stream_id: String,
    ) -> Result<StreamOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}",
        )
            .with_path_param("stream_id", stream_id)
            .execute(self.cfg)
            .await
    }

    /// Update a stream.
    pub async fn update(
        &self,
        stream_id: String,
        stream_in: StreamIn,
    ) -> Result<StreamOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/stream/{stream_id}",
        )
            .with_path_param("stream_id", stream_id)
            .with_body_param(stream_in)
            .execute(self.cfg)
            .await
    }

    /// Delete a stream.
    pub async fn delete(
        &self,
        stream_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/stream/{stream_id}",
        )
            .with_path_param("stream_id", stream_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Partially update a stream.
    pub async fn patch(
        &self,
        stream_id: String,
        stream_patch: StreamPatch,
    ) -> Result<StreamOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/stream/{stream_id}",
        )
            .with_path_param("stream_id", stream_id)
            .with_body_param(stream_patch)
            .execute(self.cfg)
            .await
    }
}
