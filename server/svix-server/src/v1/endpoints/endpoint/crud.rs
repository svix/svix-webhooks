use std::collections::HashSet;

use axum::{
    extract::{Extension, Path},
    Json,
};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};

use super::{EndpointIn, EndpointOut};
use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, EndpointId, EndpointIdOrUid, EndpointSecret, EventTypeName,
            EventTypeNameSet, OrganizationId,
        },
    },
    db::models::{endpoint, eventtype},
    error::{HttpError, Result, ValidationErrorItem},
    v1::utils::{
        EmptyResponse, ListResponse, ModelIn, ModelOut, Pagination, ValidatedJson, ValidatedQuery,
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
    let limit = pagination.limit;
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
    ValidatedJson(data): ValidatedJson<EndpointIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EndpointOut>)> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &permissions.org_id).await?;
    }

    let endp = if data.key.is_some() {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            ..data.into()
        }
    } else {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            key: Set(EndpointSecret::generate()?),
            ..data.into()
        }
    };
    let ret = endp.insert(db).await?;
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

    let mut app: endpoint::ActiveModel = endp.into();
    data.update_model(&mut app);

    let ret = app.update(db).await?;
    Ok(Json(ret.into()))
}

pub(super) async fn delete_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: endpoint::ActiveModel = endp.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
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
