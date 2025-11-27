// this file is @generated
use crate::{
    error::Result,
    models::*,
    Configuration,
};

#[derive(Default)]
pub struct ApplicationListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct ApplicationCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Application<'a> {
    cfg: &'a Configuration,
}

impl<'a> Application<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List of all the organization's applications.
    pub async fn list(
        &self,
        options: Option<ApplicationListOptions>,
    ) -> Result<ListResponseApplicationOut> {
        let ApplicationListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app",
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

    /// Create a new application.
    pub async fn create(
        &self,
        application_in: ApplicationIn,
        options: Option<ApplicationCreateOptions>,
    ) -> Result<ApplicationOut> {
        let ApplicationCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app",
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(application_in)
        .execute(self.cfg)
        .await
    }

    /// Create the application with the given ID, or create a new one if it
    /// doesn't exist yet.
    pub async fn get_or_create(
        &self,
        application_in: ApplicationIn,
        options: Option<ApplicationCreateOptions>,
    ) -> Result<ApplicationOut> {
        let ApplicationCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app",
        )
        .with_query_param(
            "get_if_exists",
            "true".to_owned(),
        )
        .with_optional_header_param(
            "idempotency-key",
            idempotency_key,
        )
        .with_body_param(application_in)
        .execute(self.cfg)
        .await
    }

    /// Get an application.
    pub async fn get(
        &self,
        app_id: String,
    ) -> Result<ApplicationOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .execute(self.cfg)
        .await
    }

    /// Update an application.
    pub async fn update(
        &self,
        app_id: String,
        application_in: ApplicationIn,
    ) -> Result<ApplicationOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/app/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_body_param(application_in)
        .execute(self.cfg)
        .await
    }

    /// Delete an application.
    pub async fn delete(
        &self,
        app_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/app/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Partially update an application.
    pub async fn patch(
        &self,
        app_id: String,
        application_patch: ApplicationPatch,
    ) -> Result<ApplicationOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/app/{app_id}",
        )
        .with_path_param(
            "app_id", app_id,
        )
        .with_body_param(application_patch)
        .execute(self.cfg)
        .await
    }
}
