// this file is @generated
use super::DestinationTransformation;
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct DestinationListOptions {
    /// Limit the number of returned items
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct DestinationCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Destination<'a> {
    cfg: &'a Configuration,
}

impl<'a> Destination<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn transformation(&self) -> DestinationTransformation<'a> {
        DestinationTransformation::new(self.cfg)
    }

    /// List of all the application's destinations.
    pub async fn list(
        &self,
        app_id: String,
        options: Option<DestinationListOptions>,
    ) -> Result<ListResponseDestinationOut> {
        let DestinationListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::GET, "/api/v1/app/{app_id}/destination")
            .with_path_param("app_id", app_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Creates a new destination.
    pub async fn create(
        &self,
        app_id: String,
        destination_in: DestinationIn,
        options: Option<DestinationCreateOptions>,
    ) -> Result<DestinationOut> {
        let DestinationCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/app/{app_id}/destination")
            .with_path_param("app_id", app_id)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(destination_in)
            .execute(self.cfg)
            .await
    }

    /// Get a destination by id or uid.
    pub async fn get(&self, app_id: String, destination_id: String) -> Result<DestinationOut> {
        crate::request::Request::new(
            http::Method::GET,
            "/api/v1/app/{app_id}/destination/{destination_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .execute(self.cfg)
        .await
    }

    /// Create or update a destination.
    pub async fn upsert(
        &self,
        app_id: String,
        destination_id: String,
        destination_in: DestinationIn,
    ) -> Result<DestinationOut> {
        crate::request::Request::new(
            http::Method::PUT,
            "/api/v1/app/{app_id}/destination/{destination_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .with_body_param(destination_in)
        .execute(self.cfg)
        .await
    }

    /// Delete a destination.
    pub async fn delete(&self, app_id: String, destination_id: String) -> Result<()> {
        crate::request::Request::new(
            http::Method::DELETE,
            "/api/v1/app/{app_id}/destination/{destination_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially update a destination.
    pub async fn patch(
        &self,
        app_id: String,
        destination_id: String,
        destination_patch: DestinationPatch,
    ) -> Result<DestinationOut> {
        crate::request::Request::new(
            http::Method::PATCH,
            "/api/v1/app/{app_id}/destination/{destination_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("destination_id", destination_id)
        .with_body_param(destination_patch)
        .execute(self.cfg)
        .await
    }
}
