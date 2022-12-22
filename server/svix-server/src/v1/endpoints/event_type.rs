// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{permissions, types::EventTypeName},
    ctx,
    db::models::eventtype,
    error::{HttpError, Result},
    v1::utils::{
        api_not_implemented,
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired, EmptyResponse,
        ListResponse, ModelIn, ModelOut, Pagination, PaginationLimit, ValidatedJson,
        ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use serde::{Deserialize, Serialize};
use svix_server_derive::{ModelIn, ModelOut};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
pub struct EventTypeIn {
    pub name: EventTypeName,
    #[validate(custom = "validate_no_control_characters")]
    pub description: String,
    #[serde(default, rename = "archived")]
    pub deleted: bool,
    pub schemas: Option<eventtype::Schema>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EventTypeIn {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EventTypeIn {
            name,
            description,
            deleted,
            schemas,
        } = self;

        model.name = Set(name);
        model.description = Set(description);
        model.deleted = Set(deleted);
        model.schemas = Set(schemas);
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct EventTypeUpdate {
    #[validate(custom = "validate_no_control_characters")]
    description: String,
    #[serde(default, rename = "archived")]
    deleted: bool,
    schemas: Option<eventtype::Schema>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EventTypeUpdate {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EventTypeUpdate {
            description,
            deleted,
            schemas,
        } = self;

        model.description = Set(description);
        model.deleted = Set(deleted);
        model.schemas = Set(schemas);
    }
}

#[derive(Deserialize, ModelIn, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
struct EventTypePatch {
    #[serde(default, skip_serializing_if = "UnrequiredField::is_absent")]
    #[validate(custom = "validate_no_control_characters_unrequired")]
    description: UnrequiredField<String>,

    #[serde(
        default,
        rename = "archived",
        skip_serializing_if = "UnrequiredField::is_absent"
    )]
    deleted: UnrequiredField<bool>,

    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    schemas: UnrequiredNullableField<eventtype::Schema>,
}

impl ModelIn for EventTypePatch {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EventTypePatch {
            description,
            deleted,
            schemas,
        } = self;

        patch_field_non_nullable!(model, description);
        patch_field_non_nullable!(model, deleted);
        patch_field_nullable!(model, schemas);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
pub struct EventTypeOut {
    pub name: EventTypeName,
    pub description: String,
    #[serde(rename = "archived")]
    pub deleted: bool,
    pub schemas: Option<eventtype::Schema>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EventTypeOut {
    fn without_payload(model: eventtype::Model) -> Self {
        Self {
            schemas: None,
            ..model.into()
        }
    }
}

// FIXME: This can and should be a derive macro
impl From<eventtype::Model> for EventTypeOut {
    fn from(model: eventtype::Model) -> Self {
        Self {
            name: model.name,
            description: model.description,
            deleted: model.deleted,
            schemas: model.schemas,

            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListFetchOptions {
    #[serde(default)]
    pub include_archived: bool,
    #[serde(default)]
    pub with_content: bool,
}

async fn list_event_types(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<EventTypeName>>,
    fetch_options: ValidatedQuery<ListFetchOptions>,
    permissions::ReadAll { org_id }: permissions::ReadAll,
) -> Result<Json<ListResponse<EventTypeOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = eventtype::Entity::secure_find(org_id)
        .order_by_asc(eventtype::Column::Name)
        .limit(limit + 1);

    if !fetch_options.include_archived {
        query = query.filter(eventtype::Column::Deleted.eq(false));
    }

    if let Some(iterator) = iterator {
        query = query.filter(eventtype::Column::Name.gt(iterator));
    }

    Ok(Json(EventTypeOut::list_response_no_prev(
        ctx!(query.all(db).await)?
            .into_iter()
            .map(|x| {
                if !fetch_options.with_content {
                    EventTypeOut::without_payload(x)
                } else {
                    x.into()
                }
            })
            .collect(),
        limit as usize,
    )))
}

async fn create_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypeIn>,
) -> Result<(StatusCode, Json<EventTypeOut>)> {
    let evtype = ctx!(
        eventtype::Entity::secure_find_by_name(org_id.clone(), data.name.to_owned())
            .one(db)
            .await
    )?;
    let ret = match evtype {
        Some(evtype) => {
            if evtype.deleted {
                let mut evtype: eventtype::ActiveModel = evtype.into();
                evtype.deleted = Set(false);
                data.update_model(&mut evtype);
                ctx!(evtype.update(db).await)?
            } else {
                return Err(HttpError::conflict(
                    Some("event_type_exists".to_owned()),
                    Some("An event_type with this name already exists".to_owned()),
                )
                .into());
            }
        }
        None => {
            let evtype = eventtype::ActiveModel {
                org_id: Set(org_id),
                ..data.into()
            };
            ctx!(evtype.insert(db).await)?
        }
    };
    Ok((StatusCode::CREATED, Json(ret.into())))
}

async fn get_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions::ReadAll { org_id }: permissions::ReadAll,
) -> Result<Json<EventTypeOut>> {
    let evtype = ctx!(
        eventtype::Entity::secure_find_by_name(org_id, evtype_name)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(evtype.into()))
}

async fn update_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypeUpdate>,
) -> Result<(StatusCode, Json<EventTypeOut>)> {
    let evtype = ctx!(
        eventtype::Entity::secure_find_by_name(org_id.clone(), evtype_name.clone())
            .one(db)
            .await
    )?;

    match evtype {
        Some(evtype) => {
            let mut evtype: eventtype::ActiveModel = evtype.into();
            data.update_model(&mut evtype);
            let ret = ctx!(evtype.update(db).await)?;

            Ok((StatusCode::OK, Json(ret.into())))
        }
        None => {
            let ret = ctx!(
                eventtype::ActiveModel {
                    org_id: Set(org_id),
                    name: Set(evtype_name),
                    ..data.into()
                }
                .insert(db)
                .await
            )?;

            Ok((StatusCode::CREATED, Json(ret.into())))
        }
    }
}

async fn patch_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypePatch>,
) -> Result<Json<EventTypeOut>> {
    let evtype = ctx!(
        eventtype::Entity::secure_find_by_name(org_id, evtype_name)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    data.update_model(&mut evtype);

    let ret = ctx!(evtype.update(db).await)?;
    Ok(Json(ret.into()))
}

async fn delete_event_type(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    permissions::Organization { org_id }: permissions::Organization,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let evtype = ctx!(
        eventtype::Entity::secure_find_by_name(org_id, evtype_name)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    evtype.deleted = Set(true);
    ctx!(evtype.update(db).await)?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/event-type/",
            post(create_event_type).get(list_event_types),
        )
        .route(
            "/event-type/:event_type_name/",
            get(get_event_type)
                .put(update_event_type)
                .patch(patch_event_type)
                .delete(delete_event_type),
        )
        .route(
            "/event-type/schema/generate-example/",
            post(api_not_implemented),
        )
}

#[cfg(test)]
mod tests {

    use super::ListFetchOptions;
    use serde_json::json;

    #[test]
    fn test_list_fetch_options_default() {
        let l: ListFetchOptions = serde_json::from_value(json!({})).unwrap();
        assert!(!l.include_archived);
        assert!(!l.with_content);
    }
}
