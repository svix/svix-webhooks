use super::PostOptions;
use crate::{apis::event_type_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EventTypeListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub with_content: Option<bool>,
    pub include_archived: Option<bool>,
}

pub struct EventType<'a> {
    cfg: &'a Configuration,
}

impl<'a> EventType<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        options: Option<EventTypeListOptions>,
    ) -> Result<ListResponseEventTypeOut> {
        let EventTypeListOptions {
            iterator,
            limit,
            with_content,
            include_archived,
        } = options.unwrap_or_default();
        event_type_api::v1_period_event_type_period_list(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodListParams {
                iterator,
                limit,
                with_content,
                include_archived,
                order: None,
            },
        )
        .await
    }

    pub async fn create(
        &self,
        event_type_in: EventTypeIn,
        options: Option<PostOptions>,
    ) -> Result<EventTypeOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        event_type_api::v1_period_event_type_period_create(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodCreateParams {
                event_type_in,
                idempotency_key,
            },
        )
        .await
    }

    pub async fn get(&self, event_type_name: String) -> Result<EventTypeOut> {
        event_type_api::v1_period_event_type_period_get(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodGetParams { event_type_name },
        )
        .await
    }

    pub async fn update(
        &self,
        event_type_name: String,
        event_type_update: EventTypeUpdate,
    ) -> Result<EventTypeOut> {
        event_type_api::v1_period_event_type_period_update(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodUpdateParams {
                event_type_name,
                event_type_update,
            },
        )
        .await
    }

    pub async fn patch(
        &self,
        event_type_name: String,
        event_type_patch: EventTypePatch,
    ) -> Result<EventTypeOut> {
        event_type_api::v1_period_event_type_period_patch(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodPatchParams {
                event_type_name,
                event_type_patch,
            },
        )
        .await
    }

    pub async fn delete(&self, event_type_name: String) -> Result<()> {
        event_type_api::v1_period_event_type_period_delete(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodDeleteParams {
                event_type_name,
                expunge: None,
            },
        )
        .await
    }

    pub async fn import_openapi(
        &self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: Option<PostOptions>,
    ) -> Result<EventTypeImportOpenApiOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        event_type_api::v1_period_event_type_period_import_openapi(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodImportOpenapiParams {
                event_type_import_open_api_in,
                idempotency_key,
            },
        )
        .await
    }
}
