use super::PostOptions;
use crate::{apis::event_type_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// When `true` archived (deleted but not expunged) items are included in
    /// the response
    pub include_archived: Option<bool>,

    /// When `true` the full item (including the schema) is included in the
    /// response
    pub with_content: Option<bool>,
}

pub struct EventType<'a> {
    cfg: &'a Configuration,
}

impl<'a> EventType<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Return the list of event types.
    pub async fn list(
        &self,
        options: Option<EventTypeListOptions>,
    ) -> Result<ListResponseEventTypeOut> {
        let EventTypeListOptions {
            limit,
            iterator,
            include_archived,
            with_content,
        } = options.unwrap_or_default();

        event_type_api::v1_period_event_type_period_list(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodListParams {
                limit,
                iterator,
                // FIXME: not included for backwards compatibility, for now
                order: None,
                include_archived,
                with_content,
            },
        )
        .await
    }

    /// Create new or unarchive existing event type.
    ///
    /// Unarchiving an event type will allow endpoints to filter on it and
    /// messages to be sent with it. Endpoints filtering on the event type
    /// before archival will continue to filter on it. This operation does
    /// not preserve the description and schemas.
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

    /// Get an event type.
    pub async fn get(&self, event_type_name: String) -> Result<EventTypeOut> {
        event_type_api::v1_period_event_type_period_get(
            self.cfg,
            event_type_api::V1PeriodEventTypePeriodGetParams { event_type_name },
        )
        .await
    }

    /// Update an event type.
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

    /// Partially update an event type.
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

    /// Archive an event type.
    ///
    /// Endpoints already configured to filter on an event type will continue to
    /// do so after archival. However, new messages can not be sent with it
    /// and endpoints can not filter on it. An event type can be unarchived
    /// with the create operation.
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

    /// Given an OpenAPI spec, create new or update existing event types.
    ///
    /// If an existing `archived` event type is updated, it will be unarchived.
    ///
    /// The importer will convert all webhooks found in the either the
    /// `webhooks` or `x-webhooks` top-level.
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
