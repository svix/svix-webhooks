// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct StreamingSinkListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct StreamingSinkCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamingSinkRotateSecretOptions {
    pub idempotency_key: Option<String>,
}

pub struct StreamingSink<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingSink<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List of all the stream's sinks.
    pub async fn list(
        &self,
        stream_id: String,
        options: Option<StreamingSinkListOptions>,
    ) -> Result<ListResponseStreamSinkOut> {
        let StreamingSinkListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_optional_query_param(
            "limit", limit,
        )
        .with_optional_query_param(
            "iterator", iterator,
        )
        .with_optional_query_param(
            "order", order,
        )
        .execute(self.cfg)
        .await
    }

    /// Creates a new sink.
    pub async fn create(
        &self,
        stream_id: String,
        stream_sink_in: StreamSinkIn,
        options: Option<StreamingSinkCreateOptions>,
    ) -> Result<StreamSinkOut> {
        let StreamingSinkCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/stream/{stream_id}/sink",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(stream_sink_in)
        .execute(self.cfg)
        .await
    }

    /// Get a sink by id or uid.
    pub async fn get(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<StreamSinkOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update a sink.
    pub async fn update(
        &self,
        stream_id: String,
        sink_id: String,
        stream_sink_in: StreamSinkIn,
    ) -> Result<StreamSinkOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/stream/{stream_id}/sink/{sink_id}",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .with_body_param(stream_sink_in)
        .execute(self.cfg)
        .await
    }

    /// Delete a sink.
    pub async fn delete(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/stream/{stream_id}/sink/{sink_id}",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially update a sink.
    pub async fn patch(
        &self,
        stream_id: String,
        sink_id: String,
        stream_sink_patch: StreamSinkPatch,
    ) -> Result<StreamSinkOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/stream/{stream_id}/sink/{sink_id}",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .with_body_param(stream_sink_patch)
        .execute(self.cfg)
        .await
    }

    /// Get the sink's signing secret (only supported for http sinks)
    ///
    /// This is used to verify the authenticity of the delivery.
    ///
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    pub async fn get_secret(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<SinkSecretOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/secret",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Rotates the signing secret (only supported for http sinks).
    pub async fn rotate_secret(
        &self,
        stream_id: String,
        sink_id: String,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: Option<StreamingSinkRotateSecretOptions>,
    ) -> Result<EmptyResponse> {
        let StreamingSinkRotateSecretOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(endpoint_secret_rotate_in)
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this sink.
    pub async fn transformation_partial_update(
        &self,
        stream_id: String,
        sink_id: String,
        sink_transform_in: SinkTransformIn,
    ) -> Result<EmptyResponse> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
        )
        .with_path_param(
            "stream_id",
            stream_id,
        )
        .with_path_param(
            "sink_id", sink_id,
        )
        .with_body_param(sink_transform_in)
        .execute(self.cfg)
        .await
    }
}
