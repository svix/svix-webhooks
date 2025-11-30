// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,

    /// When `true` archived (deleted but not expunged) items are included in the response.
    pub include_archived: Option<bool>,

    /// When `true` the full item (including the schema) is included in the response.
    pub with_content: Option<bool>,
}

#[derive(Default)]
pub struct EventTypeCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EventTypeImportOpenapiOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct EventTypeDeleteOptions {
    /// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
    pub expunge: Option<bool>,
}

pub struct EventType<'a> {
    cfg: &'a Configuration,
}

impl<'a> EventType<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// Return the list of event types.
    pub async fn list(
        &self,
        options: Option<EventTypeListOptions>,
    ) -> Result<ListResponseEventTypeOut> {
        let EventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/event-type",
        )
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .with_optional_query_param("include_archived", include_archived)
            .with_optional_query_param("with_content", with_content)
            .execute(self.cfg)
            .await
    }

    /// Create new or unarchive existing event type.
    ///
    /// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
    /// Endpoints filtering on the event type before archival will continue to filter on it.
    /// This operation does not preserve the description and schemas.
    pub async fn create(
        &self,
        event_type_in: EventTypeIn,
        options: Option<EventTypeCreateOptions>,
    ) -> Result<EventTypeOut> {
        let EventTypeCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/event-type",
        )
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(event_type_in)
            .execute(self.cfg)
            .await
    }

    /// Given an OpenAPI spec, create new or update existing event types.
    /// If an existing `archived` event type is updated, it will be unarchived.
    ///
    /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    /// top-level.
    pub async fn import_openapi(
        &self,
        event_type_import_open_api_in: EventTypeImportOpenApiIn,
        options: Option<EventTypeImportOpenapiOptions>,
    ) -> Result<EventTypeImportOpenApiOut> {
        let EventTypeImportOpenapiOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/event-type/import/openapi",
        )
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(event_type_import_open_api_in)
            .execute(self.cfg)
            .await
    }

    /// Get an event type.
    pub async fn get(
        &self,
        event_type_name: String,
    ) -> Result<EventTypeOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/event-type/{event_type_name}",
        )
            .with_path_param("event_type_name", event_type_name)
            .execute(self.cfg)
            .await
    }

    /// Update an event type.
    pub async fn update(
        &self,
        event_type_name: String,
        event_type_update: EventTypeUpdate,
    ) -> Result<EventTypeOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/event-type/{event_type_name}",
        )
            .with_path_param("event_type_name", event_type_name)
            .with_body_param(event_type_update)
            .execute(self.cfg)
            .await
    }

    /// Archive an event type.
    ///
    /// Endpoints already configured to filter on an event type will continue to do so after archival.
    /// However, new messages can not be sent with it and endpoints can not filter on it.
    /// An event type can be unarchived with the
    /// [create operation](#operation/create_event_type_api_v1_event_type__post).
    pub async fn delete(
        &self,
        event_type_name: String,
        options: Option<EventTypeDeleteOptions>,
    ) -> Result<()> {
        let EventTypeDeleteOptions {
            expunge,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/event-type/{event_type_name}",
        )
            .with_path_param("event_type_name", event_type_name)
            .with_optional_query_param("expunge", expunge)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Partially update an event type.
    pub async fn patch(
        &self,
        event_type_name: String,
        event_type_patch: EventTypePatch,
    ) -> Result<EventTypeOut> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/event-type/{event_type_name}",
        )
            .with_path_param("event_type_name", event_type_name)
            .with_body_param(event_type_patch)
            .execute(self.cfg)
            .await
    }
}
