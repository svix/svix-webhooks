use std::{collections::HashSet, mem};

use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{entity::prelude::*, ActiveValue::Set, QuerySelect, TransactionTrait};
use svix_server_derive::aide_annotate;
use url::Url;

use self::hack::EventTypeNameResult;
use super::{EndpointIn, EndpointOut, EndpointPatch, EndpointUpdate};
use crate::{
    cfg::Configuration,
    core::{
        operational_webhooks::{EndpointEvent, OperationalWebhook, OperationalWebhookSender},
        permissions,
        types::{EndpointId, EventTypeName, EventTypeNameSet, OrganizationId},
    },
    db::models::{application, endpoint, endpointmetadata, eventtype},
    error::{http_error_on_conflict, HttpError, Result, Traceable, ValidationErrorItem},
    v1::utils::{
        apply_pagination,
        patch::{patch_field_non_nullable, UnrequiredField, UnrequiredNullableField},
        ApplicationEndpointPath, ApplicationPath, JsonStatus, JsonStatusUpsert, ListResponse,
        ModelIn, ModelOut, NoContent, Ordering, Pagination, PaginationLimit, ReversibleIterator,
        ValidatedJson, ValidatedQuery,
    },
    AppState,
};

/// List the application's endpoints.
#[aide_annotate(op_id = "v1.endpoint.list")]
pub(super) async fn list_endpoints(
    State(AppState { ref db, .. }): State<AppState>,
    _: Path<ApplicationPath>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination<ReversibleIterator<EndpointId>>>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<EndpointOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator;
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let query = apply_pagination(
        endpoint::Entity::secure_find(app.id),
        endpoint::Column::Id,
        limit,
        iterator,
        pagination.order.unwrap_or(Ordering::Descending),
    );

    let results = query
        .find_also_related(endpointmetadata::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|(endp, metadata)| {
            let metadata = metadata.map(|m| m.data).unwrap_or_default();
            (endp, metadata).into()
        })
        .collect();

    Ok(Json(EndpointOut::list_response(
        results,
        limit as usize,
        is_prev,
    )))
}

async fn create_endp_from_data(
    db: &DatabaseConnection,
    cfg: &Configuration,
    op_webhooks: &OperationalWebhookSender,
    app: application::Model,
    mut data: EndpointIn,
) -> Result<(endpoint::Model, endpointmetadata::Model)> {
    let key = data.key_take_or_generate(&cfg.encryption, &cfg.default_signature_type)?;

    let mut endp = endpoint::ActiveModel::new(app.id, key);
    let metadata =
        endpointmetadata::ActiveModel::new(endp.id.clone().unwrap(), mem::take(&mut data.metadata));
    data.update_model(&mut endp);

    let (endp, metadata) = {
        let txn = db.begin().await?;
        let endp = endp.insert(&txn).await.map_err(http_error_on_conflict)?;
        let metadata = metadata.upsert_or_delete(&txn).await.trace()?;
        txn.commit().await?;
        (endp, metadata)
    };

    op_webhooks
        .send_operational_webhook(
            &app.org_id,
            OperationalWebhook::EndpointCreated(EndpointEvent::new(app.uid.as_ref(), &endp)),
        )
        .await?;

    Ok((endp, metadata))
}

/// Create a new endpoint for the application.
///
/// When `secret` is `null` the secret is automatically generated (recommended)
#[aide_annotate(op_id = "v1.endpoint.create")]
pub(super) async fn create_endpoint(
    State(AppState {
        ref db,
        ref cfg,
        op_webhooks,
        ..
    }): State<AppState>,
    _: Path<ApplicationPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
) -> Result<JsonStatus<201, EndpointOut>> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &app.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let (endp, metadata) = create_endp_from_data(db, cfg, &op_webhooks, app, data)
        .await
        .trace()?;

    Ok(JsonStatus((endp, metadata.data).into()))
}

/// Get an endpoint.
#[aide_annotate(op_id = "v1.endpoint.get")]
pub(super) async fn get_endpoint(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<EndpointOut>> {
    let (endp, metadata) = endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
        .find_also_related(endpointmetadata::Entity)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let metadata = metadata.map(|m| m.data).unwrap_or_default();

    Ok(Json((endp, metadata).into()))
}

async fn update_endp_from_data(
    db: &DatabaseConnection,
    op_webhooks: &OperationalWebhookSender,
    app: application::Model,
    endp: endpoint::ActiveModel,
    metadata: endpointmetadata::ActiveModel,
) -> Result<(endpoint::Model, endpointmetadata::Model)> {
    let (endp, metadata) = {
        let txn = db.begin().await?;
        let endp = endp.update(&txn).await.map_err(http_error_on_conflict)?;
        let metadata = metadata.upsert_or_delete(&txn).await.trace()?;
        txn.commit().await?;
        (endp, metadata)
    };

    let app_uid = app.uid;
    op_webhooks
        .send_operational_webhook(
            &app.org_id,
            OperationalWebhook::EndpointUpdated(EndpointEvent::new(app_uid.as_ref(), &endp)),
        )
        .await?;

    Ok((endp, metadata))
}

/// Update an endpoint.
#[aide_annotate(op_id = "v1.endpoint.update")]
pub(super) async fn update_endpoint(
    State(AppState {
        ref db,
        ref cfg,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(mut data): ValidatedJson<EndpointUpdate>,
) -> Result<JsonStatusUpsert<EndpointOut>> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &app.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let models = endpoint::ActiveModel::fetch_with_metadata(db, app.id.clone(), endpoint_id)
        .await
        .trace()?;

    if let Some((mut endp, mut metadata)) = models {
        metadata.data = Set(mem::take(&mut data.metadata));
        data.update_model(&mut endp);
        let (endp, metadata) = update_endp_from_data(db, op_webhooks, app, endp, metadata)
            .await
            .trace()?;
        Ok(JsonStatusUpsert::Updated((endp, metadata.data).into()))
    } else {
        let data = data.into_in_with_default_key();
        let (endp, metadata) = create_endp_from_data(db, cfg, op_webhooks, app, data)
            .await
            .trace()?;
        Ok(JsonStatusUpsert::Created((endp, metadata.data).into()))
    }
}

/// Partially update an endpoint.
#[aide_annotate]
pub(super) async fn patch_endpoint(
    State(AppState {
        ref db,
        cfg,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointPatch>,
) -> Result<Json<EndpointOut>> {
    if let UnrequiredNullableField::Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &app.org_id).await?;
    }
    if let UnrequiredField::Some(url) = &data.url {
        validate_endpoint_url(url, cfg.endpoint_https_only)?;
    }

    let (mut endp, mut metadata) =
        endpoint::ActiveModel::fetch_with_metadata(db, app.id.clone(), endpoint_id)
            .await?
            .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut patch_data = data; // need to alias so we can use data for `patch_field_non_nullable!`

    let data = mem::take(&mut patch_data.metadata);
    patch_field_non_nullable!(metadata, data);
    patch_data.update_model(&mut endp);
    let (endp, metadata) = update_endp_from_data(db, op_webhooks, app, endp, metadata)
        .await
        .trace()?;

    Ok(Json((endp, metadata.data).into()))
}

/// Delete an endpoint.
#[aide_annotate(op_id = "v1.endpoint.delete")]
pub(super) async fn delete_endpoint(
    State(AppState {
        ref db,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<NoContent> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    // Cloning the ID/UID out of endp before it's consumed below
    let endpoint_id = endp.id.clone();
    let endpoint_uid = endp.uid.clone();

    let mut endp: endpoint::ActiveModel = endp.into();
    endp.deleted = Set(true);
    endp.uid = Set(None); // We don't want deleted UIDs to clash
    endp.update(db).await?;

    op_webhooks
        .send_operational_webhook(
            &app.org_id,
            OperationalWebhook::EndpointDeleted(EndpointEvent {
                app_id: app.id,
                app_uid: app.uid,
                endpoint_id,
                endpoint_uid,
            }),
        )
        .await?;

    Ok(NoContent)
}

/// This module is here so that our Result override doesn't conflict
mod hack {
    use sea_orm::FromQueryResult;

    use crate::core::types::EventTypeName;

    #[derive(Debug, FromQueryResult)]
    pub struct EventTypeNameResult {
        pub name: EventTypeName,
    }
}

async fn validate_event_types(
    db: &DatabaseConnection,
    event_types_ids: &EventTypeNameSet,
    org_id: &OrganizationId,
) -> Result<()> {
    let event_types: Vec<EventTypeNameResult> = eventtype::Entity::secure_find(org_id.clone())
        .filter(eventtype::Column::Deleted.eq(false))
        .select_only()
        .column(eventtype::Column::Name)
        .into_model::<EventTypeNameResult>()
        .all(db)
        .await?;
    let event_types: HashSet<EventTypeName> =
        HashSet::from_iter(event_types.into_iter().map(|x| x.name));
    let missing: Vec<&EventTypeName> = event_types_ids
        .0
        .iter()
        .filter(|x| !event_types.contains(x))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        let missing = missing
            .into_iter()
            .map(|x| &(x.0[..]))
            .collect::<Vec<&str>>()
            .join(", ");
        Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "filterTypes".to_owned()],
            msg: format!("The following event types don't exist: {missing}"),
            ty: "value_error".to_owned(),
        }])
        .into())
    }
}

fn validate_endpoint_url(url: &Url, https_only: bool) -> Result<()> {
    if !https_only {
        return Ok(());
    }

    let scheme = url.scheme();
    if scheme == "https" {
        Ok(())
    } else {
        Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "url".to_owned()],
            msg: "Endpoint URL schemes must be https when endpoint_https_only is set.".to_owned(),
            ty: "value_error".to_owned(),
        }])
        .into())
    }
}
