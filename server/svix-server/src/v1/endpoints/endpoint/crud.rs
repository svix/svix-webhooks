use std::{collections::HashSet, mem};

use axum::{
    extract::{Path, State},
    Json,
};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, TransactionTrait};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use url::Url;

use super::{EndpointIn, EndpointOut, EndpointPatch};
use crate::{
    cfg::Configuration,
    core::{
        operational_webhooks::{EndpointEvent, OperationalWebhook, OperationalWebhookSender},
        permissions,
        types::{EndpointId, EventTypeName, EventTypeNameSet, OrganizationId},
    },
    ctx,
    db::models::{application, endpoint, endpointmetadata, eventtype},
    error::{HttpError, Result, ValidationErrorItem},
    v1::utils::{
        apply_pagination,
        patch::{patch_field_non_nullable, UnrequiredField, UnrequiredNullableField},
        ApplicationEndpointPath, EmptyResponse, ListOrdering, ListResponse, ModelIn, ModelOut,
        Ordering, Pagination, PaginationLimit, ReversibleIterator, ValidatedJson, ValidatedQuery,
    },
    AppState,
};
use hack::EventTypeNameResult;

pub(super) const LIST_ENDPOINTS_DESCRIPTION: &str = "List the application's endpoints.";

pub(super) async fn list_endpoints(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination<ReversibleIterator<EndpointId>>>,
    ValidatedQuery(Ordering { order }): ValidatedQuery<Ordering>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<EndpointOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator;
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let order = order.unwrap_or(ListOrdering::Descending);
    let query = apply_pagination(
        endpoint::Entity::secure_find(app.id),
        endpoint::Column::Id,
        limit,
        iterator,
        order.clone(),
    );

    let results = ctx!(
        query
            .find_also_related(endpointmetadata::Entity)
            .all(db)
            .await
    )?
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
        let txn = ctx!(db.begin().await)?;
        let endp = ctx!(endp.insert(&txn).await)?;
        let metadata = ctx!(metadata.upsert_or_delete(&txn).await)?;
        ctx!(txn.commit().await)?;
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

pub(super) const CREATE_ENDPOINT_DESCRIPTION: &str = r#"
Create a new endpoint for the application.

When `secret` is `null` the secret is automatically generated (recommended)
"#;

pub(super) async fn create_endpoint(
    State(AppState {
        ref db,
        ref cfg,
        op_webhooks,
        ..
    }): State<AppState>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
) -> Result<(StatusCode, Json<EndpointOut>)> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &app.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let (endp, metadata) = ctx!(create_endp_from_data(db, cfg, &op_webhooks, app, data).await)?;

    Ok((StatusCode::CREATED, Json((endp, metadata.data).into())))
}

pub(super) const GET_ENDPOINT_DESCRIPTION: &str = "Get an endpoint.";

pub(super) async fn get_endpoint(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<EndpointOut>> {
    let (endp, metadata) = ctx!(
        endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
            .find_also_related(endpointmetadata::Entity)
            .one(db)
            .await
    )?
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
        let txn = ctx!(db.begin().await)?;
        let endp = ctx!(endp.update(&txn).await)?;
        let metadata = ctx!(metadata.upsert_or_delete(&txn).await)?;
        ctx!(txn.commit().await)?;
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

pub(super) const UPDATE_ENDPOINT_DESCRIPTION: &str = "Update an endpoint.";

pub(super) async fn update_endpoint(
    State(AppState {
        ref db,
        ref cfg,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(mut data): ValidatedJson<EndpointIn>,
) -> Result<(StatusCode, Json<EndpointOut>)> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &app.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let models =
        ctx!(endpoint::ActiveModel::fetch_with_metadata(db, app.id.clone(), endpoint_id).await)?;

    if let Some((mut endp, mut metadata)) = models {
        metadata.data = Set(mem::take(&mut data.metadata));
        data.update_model(&mut endp);
        let (endp, metadata) =
            ctx!(update_endp_from_data(db, op_webhooks, app, endp, metadata).await)?;
        Ok((StatusCode::OK, Json((endp, metadata.data).into())))
    } else {
        let (endp, metadata) = ctx!(create_endp_from_data(db, cfg, op_webhooks, app, data).await)?;
        Ok((StatusCode::CREATED, Json((endp, metadata.data).into())))
    }
}

pub(super) const PATCH_ENDPOINT_DESCRIPTION: &str = "Partially update an endpoint.";

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
        ctx!(endpoint::ActiveModel::fetch_with_metadata(db, app.id.clone(), endpoint_id).await)?
            .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut patch_data = data; // need to alias so we can use data for `patch_field_non_nullable!`

    let data = mem::take(&mut patch_data.metadata);
    patch_field_non_nullable!(metadata, data);
    patch_data.update_model(&mut endp);
    let (endp, metadata) = ctx!(update_endp_from_data(db, op_webhooks, app, endp, metadata).await)?;

    Ok(Json((endp, metadata.data).into()))
}

pub(super) const DELETE_ENDPOINT_DESCRIPTION: &str = "Delete an endpoint.";

pub(super) async fn delete_endpoint(
    State(AppState {
        ref db,
        ref op_webhooks,
        ..
    }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let endp = ctx!(
        endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;

    // Cloning the ID/UID out of endp before it's consumed below
    let endpoint_id = endp.id.clone();
    let endpoint_uid = endp.uid.clone();

    let mut endp: endpoint::ActiveModel = endp.into();
    endp.deleted = Set(true);
    endp.uid = Set(None); // We don't want deleted UIDs to clash
    ctx!(endp.update(db).await)?;

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

    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
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
    let event_types: Vec<EventTypeNameResult> = ctx!(
        eventtype::Entity::secure_find(org_id.clone())
            .filter(eventtype::Column::Deleted.eq(false))
            .select_only()
            .column(eventtype::Column::Name)
            .into_model::<EventTypeNameResult>()
            .all(db)
            .await
    )?;
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
