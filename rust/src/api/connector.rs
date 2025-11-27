// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct ConnectorListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct ConnectorCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Connector<'a> {
    cfg: &'a Configuration,
}

impl<'a> Connector<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List all connectors for an application.
    pub async fn list(
        &self,
        options: Option<ConnectorListOptions>,
    ) -> Result<ListResponseConnectorOut> {
        let ConnectorListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/connector",
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

    /// Create a new connector.
    pub async fn create(
        &self,
        connector_in: ConnectorIn,
        options: Option<ConnectorCreateOptions>,
    ) -> Result<ConnectorOut> {
        let ConnectorCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/connector",
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(connector_in)
        .execute(self.cfg)
        .await
    }

    /// Get a connector.
    pub async fn get(
        &self,
        connector_id: String,
    ) -> Result<ConnectorOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/connector/{connector_id}",
        )
        .with_path_param(
            "connector_id",
            connector_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update a connector.
    pub async fn update(
        &self,
        connector_id: String,
        connector_update: ConnectorUpdate,
    ) -> Result<ConnectorOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/connector/{connector_id}",
        )
        .with_path_param(
            "connector_id",
            connector_id,
        )
        .with_body_param(connector_update)
        .execute(self.cfg)
        .await
    }

    /// Delete a connector.
    pub async fn delete(
        &self,
        connector_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/connector/{connector_id}",
        )
        .with_path_param(
            "connector_id",
            connector_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially update a connector.
    pub async fn patch(
        &self,
        connector_id: String,
        connector_patch: ConnectorPatch,
    ) -> Result<ConnectorOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/connector/{connector_id}",
        )
        .with_path_param(
            "connector_id",
            connector_id,
        )
        .with_body_param(connector_patch)
        .execute(self.cfg)
        .await
    }
}
