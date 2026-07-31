// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct StreamingSinkTransformation<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingSinkTransformation<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get the transformation code associated with this sink.
    pub async fn get(&self, stream_id: String, sink_id: String) -> Result<SinkTransformationOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
        )
        .with_path_param("stream_id", stream_id)
        .with_path_param("sink_id", sink_id)
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this sink.
    pub async fn patch(
        &self,
        stream_id: String,
        sink_id: String,
        sink_transform_in: SinkTransformIn,
    ) -> Result<EmptyResponse> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation",
        )
        .with_path_param("stream_id", stream_id)
        .with_path_param("sink_id", sink_id)
        .with_body_param(sink_transform_in)
        .execute(self.cfg)
        .await
    }
}
