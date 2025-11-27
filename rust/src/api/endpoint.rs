// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct EndpointListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct EndpointCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointRecoverOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointReplayMissingOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointRotateSecretOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointSendExampleOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EndpointGetStatsOptions {
    /// Filter the range to data starting from this date.
    ///
    /// RFC3339 date string.
    pub since: Option<String>,

    /// Filter the range to data ending by this date.
    ///
    /// RFC3339 date string.
    pub until: Option<String>,
}

pub struct Endpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> Endpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List the application's endpoints.
    pub async fn list(
        &self,
        app_id: String,
        options: Option<EndpointListOptions>,
    ) -> Result<ListResponseEndpointOut> {
        let EndpointListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint",
        )
        .with_path_param(
            "app_id", app_id,
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

    /// Create a new endpoint for the application.
    ///
    /// When `secret` is `null` the secret is automatically generated
    /// (recommended).
    pub async fn create(
        &self,
        app_id: String,
        endpoint_in: EndpointIn,
        options: Option<EndpointCreateOptions>,
    ) -> Result<EndpointOut> {
        let EndpointCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/endpoint",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(endpoint_in)
        .execute(self.cfg)
        .await
    }

    /// Get an endpoint.
    pub async fn get(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update an endpoint.
    pub async fn update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_update: EndpointUpdate,
    ) -> Result<EndpointOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_update)
        .execute(self.cfg)
        .await
    }

    /// Delete an endpoint.
    pub async fn delete(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially update an endpoint.
    pub async fn patch(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_patch: EndpointPatch,
    ) -> Result<EndpointOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_patch)
        .execute(self.cfg)
        .await
    }

    /// Get the additional headers to be sent with the webhook.
    pub async fn get_headers(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointHeadersOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Set the additional headers to be sent with the webhook.
    pub async fn update_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_in: EndpointHeadersIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_headers_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially set the additional headers to be sent with the webhook.
    pub async fn patch_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_headers_patch_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Resend all failed messages since a given time.
    ///
    /// Messages that were sent successfully, even if failed initially, are not
    /// resent.
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "endpoint.recover",
    ///   "data": {
    ///     "messagesSent": 2
    ///   }
    /// }
    /// ```
    pub async fn recover(
        &self,
        app_id: String,
        endpoint_id: String,
        recover_in: RecoverIn,
        options: Option<EndpointRecoverOptions>,
    ) -> Result<RecoverOut> {
        let EndpointRecoverOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(recover_in)
        .execute(self.cfg)
        .await
    }

    /// Replays messages to the endpoint.
    ///
    /// Only messages that were created after `since` will be sent.
    /// Messages that were previously sent to the endpoint are not resent.
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "endpoint.replay",
    ///   "data": {
    ///     "messagesSent": 2
    ///   }
    /// }
    /// ```
    pub async fn replay_missing(
        &self,
        app_id: String,
        endpoint_id: String,
        replay_in: ReplayIn,
        options: Option<EndpointReplayMissingOptions>,
    ) -> Result<ReplayOut> {
        let EndpointReplayMissingOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(replay_in)
        .execute(self.cfg)
        .await
    }

    /// Get the endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    pub async fn get_secret(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointSecretOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Rotates the endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    pub async fn rotate_secret(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: Option<EndpointRotateSecretOptions>,
    ) -> Result<()> {
        let EndpointRotateSecretOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(endpoint_secret_rotate_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Send an example message for an event.
    pub async fn send_example(
        &self,
        app_id: String,
        endpoint_id: String,
        event_example_in: EventExampleIn,
        options: Option<EndpointSendExampleOptions>,
    ) -> Result<MessageOut> {
        let EndpointSendExampleOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(event_example_in)
        .execute(self.cfg)
        .await
    }

    /// Get basic statistics for the endpoint.
    pub async fn get_stats(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<EndpointGetStatsOptions>,
    ) -> Result<EndpointStats> {
        let EndpointGetStatsOptions {
            since,
            until,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_optional_query_param(
            "since", since,
        )
        .with_optional_query_param(
            "until", until,
        )
        .execute(self.cfg)
        .await
    }

    /// Get the transformation code associated with this endpoint.
    pub async fn transformation_get(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointTransformationOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Set or unset the transformation code associated with this endpoint.
    pub async fn patch_transformation(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_patch: EndpointTransformationPatch,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_transformation_patch)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// This operation was renamed to `set-transformation`.
    #[deprecated]
    pub async fn transformation_partial_update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_path_param(
            "endpoint_id",
            endpoint_id,
        )
        .with_body_param(endpoint_transformation_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
