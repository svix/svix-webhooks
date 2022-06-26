use std::collections::HashSet;

use axum::{
    extract::{Extension, Path},
    Json,
};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use url::Url;

use super::{secrets::generate_secret, EndpointIn, EndpointOut};
use crate::{
    cfg::Configuration,
    core::{
        operational_webhooks::{EndpointEvent, OperationalWebhook, OperationalWebhookSender},
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, EndpointId, EndpointIdOrUid, EventTypeName, EventTypeNameSet,
            OrganizationId,
        },
    },
    db::models::{endpoint, eventtype},
    error::{HttpError, Result, ValidationErrorItem},
    v1::utils::{
        EmptyResponse, ListResponse, ModelIn, ModelOut, Pagination, PaginationLimit, ValidatedJson,
        ValidatedQuery,
    },
};
use hack::EventTypeNameResult;

pub(super) async fn list_endpoints(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<EndpointId>>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<EndpointOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = endpoint::Entity::secure_find(app.id)
        .order_by_asc(endpoint::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(endpoint::Column::Id.gt(iterator))
    }

    Ok(Json(EndpointOut::list_response_no_prev(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

pub(super) async fn create_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(cfg): Extension<Configuration>,
    Extension(op_webhooks): Extension<OperationalWebhookSender>,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EndpointOut>)> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &permissions.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let endp = if data.key.is_some() {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            ..data.into()
        }
    } else {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            key: Set(generate_secret(&cfg.default_signature_type)?),
            ..data.into()
        }
    };
    let ret = endp.insert(db).await?;

    op_webhooks
        .send_operational_webhook(
            &permissions.org_id,
            OperationalWebhook::EndpointCreated(EndpointEvent {
                app_id: &ret.app_id,
                app_uid: app.uid.as_ref(),
                endpoint_id: &ret.id,
                endpoint_uid: ret.uid.as_ref(),
            }),
        )
        .await?;

    Ok((StatusCode::CREATED, Json(ret.into())))
}

pub(super) async fn get_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<EndpointOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(endp.into()))
}

pub(super) async fn update_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(cfg): Extension<Configuration>,
    Extension(op_webhooks): Extension<OperationalWebhookSender>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<Json<EndpointOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &permissions.org_id).await?;
    }
    validate_endpoint_url(&data.url, cfg.endpoint_https_only)?;

    let mut endp: endpoint::ActiveModel = endp.into();
    data.update_model(&mut endp);

    let ret = endp.update(db).await?;

    let app_uid = app.uid;
    op_webhooks
        .send_operational_webhook(
            &permissions.org_id,
            OperationalWebhook::EndpointUpdated(EndpointEvent {
                app_id: &ret.app_id,
                app_uid: app_uid.as_ref(),
                endpoint_id: &ret.id,
                endpoint_uid: ret.uid.as_ref(),
            }),
        )
        .await?;

    Ok(Json(ret.into()))
}

pub(super) async fn delete_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(op_webhooks): Extension<OperationalWebhookSender>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
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
            &permissions.org_id,
            OperationalWebhook::EndpointDeleted(EndpointEvent {
                app_id: &app.id,
                app_uid: app.uid.as_ref(),
                endpoint_id: &endpoint_id,
                endpoint_uid: endpoint_uid.as_ref(),
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
        Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "event_types_ids".to_owned()],
            msg: format!("The following type names don't exist: {:?}", missing),
            ty: "value_error".to_owned(),
        }])
        .into())
    }
}

fn validate_endpoint_url(url: &str, https_only: bool) -> Result<()> {
    if !https_only {
        return Ok(());
    }

    match Url::parse(url) {
        Ok(url) => {
            let scheme = url.scheme();
            if scheme == "https" {
                Ok(())
            } else {
                Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
                    loc: vec!["body".to_owned(), "url".to_owned()],
                    msg: "Endpoint URL schemes must be https when endpoint_https_only is set."
                        .to_owned(),
                    ty: "value_error".to_owned(),
                }])
                .into())
            }
        }

        Err(_) => Err(HttpError::unprocessable_entity(vec![ValidationErrorItem {
            loc: vec!["body".to_owned(), "url".to_owned()],
            msg: "Endpoint URLs must be valid".to_owned(),
            ty: "value_error".to_owned(),
        }])
        .into()),
    }
}
