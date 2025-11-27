// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct IngestEndpointListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct IngestEndpointCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct IngestEndpointRotateSecretOptions {
    pub idempotency_key: Option<String>,
}

pub struct IngestEndpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> IngestEndpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List ingest endpoints.
    pub async fn list(
        &self,
        source_id: String,
        options: Option<IngestEndpointListOptions>,
    ) -> Result<ListResponseIngestEndpointOut> {
        let IngestEndpointListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint",
        )
        .with_path_param(
            "source_id",
            source_id,
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

    /// Create an ingest endpoint.
    pub async fn create(
        &self,
        source_id: String,
        ingest_endpoint_in: IngestEndpointIn,
        options: Option<IngestEndpointCreateOptions>,
    ) -> Result<IngestEndpointOut> {
        let IngestEndpointCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/ingest/api/v1/source/{source_id}/endpoint",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(ingest_endpoint_in)
        .execute(self.cfg)
        .await
    }

    /// Get an ingest endpoint.
    pub async fn get(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<IngestEndpointOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update an ingest endpoint.
    pub async fn update(
        &self,
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_update: IngestEndpointUpdate,
    ) -> Result<IngestEndpointOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(ingest_endpoint_update)
        .execute(self.cfg)
        .await
    }

    /// Delete an ingest endpoint.
    pub async fn delete(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Get the additional headers to be sent with the ingest.
    pub async fn get_headers(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<IngestEndpointHeadersOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Set the additional headers to be sent to the endpoint.
    pub async fn update_headers(
        &self,
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_headers_in: IngestEndpointHeadersIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/headers",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(ingest_endpoint_headers_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Get an ingest endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    pub async fn get_secret(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<IngestEndpointSecretOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Rotates an ingest endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    pub async fn rotate_secret(
        &self,
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_secret_in: IngestEndpointSecretIn,
        options: Option<IngestEndpointRotateSecretOptions>,
    ) -> Result<()> {
        let IngestEndpointRotateSecretOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/secret/rotate",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(ingest_endpoint_secret_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Get the transformation code associated with this ingest endpoint.
    pub async fn get_transformation(
        &self,
        source_id: String,
        endpoint_id: String,
    ) -> Result<IngestEndpointTransformationOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this ingest
    /// endpoint.
    pub async fn set_transformation(
        &self,
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_transformation_patch: IngestEndpointTransformationPatch,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param(
            "source_id",
            source_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(ingest_endpoint_transformation_patch)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
