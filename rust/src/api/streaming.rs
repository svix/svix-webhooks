// this file is @generated
use super::{
    StreamingEventType,
    StreamingEvents,
    StreamingSink,
    StreamingStream,
};
use crate::{error::Result, models::*, Configuration};

pub struct Streaming<'a> {
    cfg: &'a Configuration,
}

impl<'a> Streaming<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    pub fn event_type(&self) -> StreamingEventType<'a> {
        StreamingEventType::new(self.cfg)
    }

    pub fn events(&self) -> StreamingEvents<'a> {
        StreamingEvents::new(self.cfg)
    }

    pub fn sink(&self) -> StreamingSink<'a> {
        StreamingSink::new(self.cfg)
    }

    pub fn stream(&self) -> StreamingStream<'a> {
        StreamingStream::new(self.cfg)
    }

    /// Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks.
    pub async fn sink_headers_get(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<EndpointHeadersOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
        )
            .with_path_param("stream_id", stream_id)
            .with_path_param("sink_id", sink_id)
            .execute(self.cfg)
            .await
    }

    /// Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks.
    pub async fn sink_headers_patch(
        &self,
        stream_id: String,
        sink_id: String,
        http_sink_headers_patch_in: HttpSinkHeadersPatchIn,
    ) -> Result<EndpointHeadersOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/headers",
        )
            .with_path_param("stream_id", stream_id)
            .with_path_param("sink_id", sink_id)
            .with_body_param(http_sink_headers_patch_in)
            .execute(self.cfg)
            .await
    }

    /// Get the transformation code associated with this sink.
    pub async fn sink_transformation_get(
        &self,
        stream_id: String,
        sink_id: String,
    ) -> Result<SinkTransformationOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
        )
            .with_path_param("stream_id", stream_id)
            .with_path_param("sink_id", sink_id)
            .execute(self.cfg)
            .await
    }
}
