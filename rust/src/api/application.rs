use super::PostOptions;
use crate::{apis::application_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ApplicationListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

pub struct Application<'a> {
    cfg: &'a Configuration,
}

impl<'a> Application<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
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

        application_api::v1_period_application_period_list(
            self.cfg,
            application_api::V1PeriodApplicationPeriodListParams {
                limit,
                iterator,
                order,
            },
        )
        .await
    }

    /// Create a new application.
    pub async fn create(
        &self,
        application_in: ApplicationIn,
        options: Option<PostOptions>,
    ) -> Result<ApplicationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        application_api::v1_period_application_period_create(
            self.cfg,
            application_api::V1PeriodApplicationPeriodCreateParams {
                application_in,
                idempotency_key,
                get_if_exists: None,
            },
        )
        .await
    }

    /// Create the application with the given ID, or create a new one if it
    /// doesn't exist yet.
    pub async fn get_or_create(
        &self,
        application_in: ApplicationIn,
        options: Option<PostOptions>,
    ) -> Result<ApplicationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        application_api::v1_period_application_period_create(
            self.cfg,
            application_api::V1PeriodApplicationPeriodCreateParams {
                application_in,
                idempotency_key,
                get_if_exists: Some(true),
            },
        )
        .await
    }

    /// Get an application.
    pub async fn get(&self, app_id: String) -> Result<ApplicationOut> {
        application_api::v1_period_application_period_get(
            self.cfg,
            application_api::V1PeriodApplicationPeriodGetParams { app_id },
        )
        .await
    }

    /// Update an application.
    pub async fn update(
        &self,
        app_id: String,
        application_in: ApplicationIn,
    ) -> Result<ApplicationOut> {
        application_api::v1_period_application_period_update(
            self.cfg,
            application_api::V1PeriodApplicationPeriodUpdateParams {
                app_id,
                application_in,
            },
        )
        .await
    }

    /// Delete an application.
    pub async fn delete(&self, app_id: String) -> Result<()> {
        application_api::v1_period_application_period_delete(
            self.cfg,
            application_api::V1PeriodApplicationPeriodDeleteParams { app_id },
        )
        .await
    }

    /// Partially update an application.
    pub async fn patch(
        &self,
        app_id: String,
        application_patch: ApplicationPatch,
    ) -> Result<ApplicationOut> {
        application_api::v1_period_application_period_patch(
            self.cfg,
            application_api::V1PeriodApplicationPeriodPatchParams {
                app_id,
                application_patch,
            },
        )
        .await
    }
}
