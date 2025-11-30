// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EnvironmentExportOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EnvironmentImportOptions {
    pub idempotency_key: Option<String>,
}

pub struct Environment<'a> {
    cfg: &'a Configuration,
}

impl<'a> Environment<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// Download a JSON file containing all org-settings and event types.
    ///
    /// Note that the schema for [`EnvironmentOut`] is subject to change. The fields
    /// herein are provided for convenience but should be treated as JSON blobs.
    pub async fn export(
        &self,
        options: Option<EnvironmentExportOptions>,
    ) -> Result<EnvironmentOut> {
        let EnvironmentExportOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/environment/export",
        )
            .with_optional_header_param("idempotency-key", idempotency_key)
            .execute(self.cfg)
            .await
    }

    /// Import a configuration into the active organization.
    ///
    /// It doesn't delete anything, only adds / updates what was passed to it.
    ///
    /// Note that the schema for [`EnvironmentIn`] is subject to change. The fields
    /// herein are provided for convenience but should be treated as JSON blobs.
    pub async fn import(
        &self,
        environment_in: EnvironmentIn,
        options: Option<EnvironmentImportOptions>,
    ) -> Result<()> {
        let EnvironmentImportOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/environment/import",
        )
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(environment_in)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }
}
