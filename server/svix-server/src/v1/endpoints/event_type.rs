// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use svix_server_derive::{aide_annotate, ModelIn, ModelOut};
use validator::Validate;

use crate::{
    core::{
        permissions,
        types::{EventTypeName, FeatureFlag},
    },
    db::models::eventtype,
    error::{http_error_on_conflict, HttpError, Result},
    v1::utils::{
        api_not_implemented, apply_pagination, openapi_desc, openapi_tag,
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired,
        EventTypeNamePath, JsonStatus, JsonStatusUpsert, ListResponse, ModelIn, ModelOut,
        NoContent, Ordering, Pagination, PaginationLimit, ReversibleIterator, ValidatedJson,
        ValidatedQuery,
    },
    AppState,
};

fn example_event_archived() -> bool {
    false
}

fn event_type_description_example() -> &'static str {
    "A user has signed up"
}

fn event_type_versioned_schemas_example() -> serde_json::Value {
    serde_json::json!({ "1": eventtype::schema_example() })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventTypeIn {
    #[validate]
    pub name: EventTypeName,
    #[validate(custom = "validate_no_control_characters")]
    #[schemars(example = "event_type_description_example")]
    pub description: String,
    #[serde(default, rename = "archived")]
    #[schemars(example = "example_event_archived")]
    pub deleted: bool,
    /// The schema for the event type for a specific version as a JSON schema.
    #[schemars(example = "event_type_versioned_schemas_example")]
    pub schemas: Option<eventtype::Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<FeatureFlag>,
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
            feature_flag,
        } = self;

        model.name = Set(name);
        model.description = Set(description);
        model.deleted = Set(deleted);
        model.schemas = Set(schemas);
        model.feature_flag = Set(feature_flag);
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct EventTypeUpdate {
    #[validate(custom = "validate_no_control_characters")]
    #[schemars(example = "event_type_description_example")]
    description: String,
    #[serde(default, rename = "archived")]
    #[schemars(example = "example_event_archived")]
    deleted: bool,
    /// The schema for the event type for a specific version as a JSON schema.
    #[schemars(example = "event_type_versioned_schemas_example")]
    schemas: Option<eventtype::Schema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    feature_flag: Option<FeatureFlag>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EventTypeUpdate {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EventTypeUpdate {
            description,
            deleted,
            schemas,
            feature_flag,
        } = self;

        model.description = Set(description);
        model.deleted = Set(deleted);
        model.schemas = Set(schemas);
        model.feature_flag = Set(feature_flag);
    }
}

#[derive(Deserialize, ModelIn, Serialize, Validate, JsonSchema)]
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

    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    feature_flag: UnrequiredNullableField<FeatureFlag>,
}

impl ModelIn for EventTypePatch {
    type ActiveModel = eventtype::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EventTypePatch {
            description,
            deleted,
            schemas,
            feature_flag,
        } = self;

        patch_field_non_nullable!(model, description);
        patch_field_non_nullable!(model, deleted);
        patch_field_nullable!(model, schemas);
        patch_field_nullable!(model, feature_flag);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventTypeOut {
    pub name: EventTypeName,
    #[schemars(example = "event_type_description_example")]
    pub description: String,
    #[serde(rename = "archived")]
    #[schemars(example = "example_event_archived", default = "example_event_archived")]
    pub deleted: bool,
    /// The schema for the event type for a specific version as a JSON schema.
    #[schemars(example = "event_type_versioned_schemas_example")]
    pub schemas: Option<eventtype::Schema>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub feature_flag: Option<FeatureFlag>,
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
            feature_flag: model.feature_flag,

            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ListFetchQueryParams {
    /// When `true` archived (deleted but not expunged) items are included in the response
    #[serde(default)]
    pub include_archived: bool,
    /// When `true` the full item (including the schema) is included in the response
    #[serde(default)]
    pub with_content: bool,
}

/// Return the list of event types.
#[aide_annotate(op_id = "v1.event-type.list")]
async fn list_event_types(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination<ReversibleIterator<EventTypeName>>>,
    fetch_options: ValidatedQuery<ListFetchQueryParams>,
    permissions::ReadAll {
        org_id,
        feature_flags,
        ..
    }: permissions::ReadAll,
) -> Result<Json<ListResponse<EventTypeOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator;
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let mut query = eventtype::Entity::secure_find(org_id);

    if !fetch_options.include_archived {
        query = query.filter(eventtype::Column::Deleted.eq(false));
    }

    if let permissions::AllowedFeatureFlags::Some(flags) = feature_flags {
        query = eventtype::Entity::filter_feature_flags(query, flags);
    }

    let query = apply_pagination(
        query,
        eventtype::Column::Name,
        limit,
        iterator,
        Ordering::Ascending,
    );

    Ok(Json(EventTypeOut::list_response(
        query
            .all(db)
            .await?
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
        is_prev,
    )))
}

/// Create new or unarchive existing event type.
///
/// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
/// Endpoints filtering on the event type before archival will continue to filter on it.
/// This operation does not preserve the description and schemas.
#[aide_annotate(op_id = "v1.event-type.create")]
async fn create_event_type(
    State(AppState { ref db, .. }): State<AppState>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypeIn>,
) -> Result<JsonStatus<201, EventTypeOut>> {
    let evtype = eventtype::Entity::secure_find_by_name(org_id.clone(), data.name.to_owned())
        .one(db)
        .await?;
    let ret = match evtype {
        Some(evtype) => {
            if evtype.deleted {
                let mut evtype: eventtype::ActiveModel = evtype.into();
                evtype.deleted = Set(false);
                data.update_model(&mut evtype);
                evtype.update(db).await?
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
            evtype.insert(db).await.map_err(http_error_on_conflict)?
        }
    };
    Ok(JsonStatus(ret.into()))
}

/// Get an event type.
#[aide_annotate(op_id = "v1.event-type.get")]
async fn get_event_type(
    State(AppState { ref db, .. }): State<AppState>,
    Path(EventTypeNamePath { event_type_name }): Path<EventTypeNamePath>,
    permissions::ReadAll {
        org_id,
        feature_flags,
        ..
    }: permissions::ReadAll,
) -> Result<Json<EventTypeOut>> {
    let mut query = eventtype::Entity::secure_find_by_name(org_id, event_type_name);
    if let permissions::AllowedFeatureFlags::Some(flags) = feature_flags {
        query = eventtype::Entity::filter_feature_flags(query, flags);
    }
    let evtype = query
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    Ok(Json(evtype.into()))
}

/// Update an event type.
#[aide_annotate(op_id = "v1.event-type.update")]
async fn update_event_type(
    State(AppState { ref db, .. }): State<AppState>,
    Path(EventTypeNamePath { event_type_name }): Path<EventTypeNamePath>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypeUpdate>,
) -> Result<JsonStatusUpsert<EventTypeOut>> {
    let evtype = eventtype::Entity::secure_find_by_name(org_id.clone(), event_type_name.clone())
        .one(db)
        .await?;

    match evtype {
        Some(evtype) => {
            let mut evtype: eventtype::ActiveModel = evtype.into();
            data.update_model(&mut evtype);
            let ret = evtype.update(db).await.map_err(http_error_on_conflict)?;

            Ok(JsonStatusUpsert::Updated(ret.into()))
        }
        None => {
            let ret = eventtype::ActiveModel {
                org_id: Set(org_id),
                name: Set(event_type_name),
                ..data.into()
            }
            .insert(db)
            .await
            .map_err(http_error_on_conflict)?;

            Ok(JsonStatusUpsert::Created(ret.into()))
        }
    }
}

/// Partially update an event type.
#[aide_annotate]
async fn patch_event_type(
    State(AppState { ref db, .. }): State<AppState>,
    Path(EventTypeNamePath { event_type_name }): Path<EventTypeNamePath>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<EventTypePatch>,
) -> Result<Json<EventTypeOut>> {
    let evtype = eventtype::Entity::secure_find_by_name(org_id, event_type_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    data.update_model(&mut evtype);

    let ret = evtype.update(db).await.map_err(http_error_on_conflict)?;
    Ok(Json(ret.into()))
}

/// Archive an event type.
///
/// Endpoints already configured to filter on an event type will continue to do so after archival.
/// However, new messages can not be sent with it and endpoints can not filter on it.
/// An event type can be unarchived with the
/// [create operation](#operation/create_event_type_api_v1_event_type__post).
#[aide_annotate(op_id = "v1.event-type.delete")]
async fn delete_event_type(
    State(AppState { ref db, .. }): State<AppState>,
    Path(EventTypeNamePath { event_type_name }): Path<EventTypeNamePath>,
    permissions::Organization { org_id }: permissions::Organization,
) -> Result<NoContent> {
    let evtype = eventtype::Entity::secure_find_by_name(org_id, event_type_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    evtype.deleted = Set(true);
    evtype.update(db).await?;
    Ok(NoContent)
}

const GENERATE_SCHEMA_EXAMPLE_DESCRIPTION: &str =
    "Generates a fake example from the given JSONSchema";

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Event Type");
    ApiRouter::new()
        .api_route_with(
            "/event-type",
            post_with(create_event_type, create_event_type_operation)
                .get_with(list_event_types, list_event_types_operation),
            &tag,
        )
        .api_route_with(
            "/event-type/:event_type_name",
            get_with(get_event_type, get_event_type_operation)
                .put_with(update_event_type, update_event_type_operation)
                .patch_with(patch_event_type, patch_event_type_operation)
                .delete_with(delete_event_type, delete_event_type_operation),
            &tag,
        )
        .api_route_with(
            "/event-type/schema/generate-example",
            post_with(
                api_not_implemented,
                openapi_desc(GENERATE_SCHEMA_EXAMPLE_DESCRIPTION),
            ),
            tag,
        )
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::ListFetchQueryParams;

    #[test]
    fn test_list_fetch_options_default() {
        let l: ListFetchQueryParams = serde_json::from_value(json!({})).unwrap();
        assert!(!l.include_archived);
        assert!(!l.with_content);
    }
}
