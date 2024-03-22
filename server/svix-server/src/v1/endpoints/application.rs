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
use futures::FutureExt;
use schemars::JsonSchema;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use svix_server_derive::{aide_annotate, ModelOut};
use validator::{Validate, ValidationError};

use crate::{
    core::{
        permissions,
        types::{metadata::Metadata, ApplicationId, ApplicationUid},
    },
    db::models::{application, applicationmetadata},
    error::{http_error_on_conflict, HttpError, Result, Traceable},
    v1::utils::{
        apply_pagination, openapi_tag,
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired,
        validation_error, ApplicationPath, JsonStatusUpsert, ListResponse, ModelIn, ModelOut,
        NoContent, Ordering, Pagination, PaginationLimit, ReversibleIterator, ValidatedJson,
        ValidatedQuery,
    },
    AppState,
};

fn application_name_example() -> &'static str {
    "My first application"
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationIn {
    #[validate(
        length(min = 1, message = "Application names must be at least one character"),
        custom = "validate_no_control_characters"
    )]
    #[schemars(example = "application_name_example")]
    pub name: String,

    #[validate(range(min = 1, message = "Application rate limits must be at least 1 if set"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,
    /// Optional unique identifier for the application
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<ApplicationUid>,

    #[serde(default)]
    pub metadata: Metadata,
}

// FIXME: This can and should be a derive macro
impl ModelIn for ApplicationIn {
    type ActiveModel = (application::ActiveModel, applicationmetadata::ActiveModel);

    fn update_model(self, (app, app_metadata): &mut Self::ActiveModel) {
        let ApplicationIn {
            name,
            rate_limit,
            uid,
            metadata,
        } = self;

        app.name = Set(name);
        app.rate_limit = Set(rate_limit.map(|x| x.into()));
        app.uid = Set(uid);
        app_metadata.data = Set(metadata);
    }
}

#[derive(Deserialize, Serialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationPatch {
    #[serde(default, skip_serializing_if = "UnrequiredField::is_absent")]
    #[validate(
        custom = "validate_name_length_patch",
        custom = "validate_no_control_characters_unrequired"
    )]
    pub name: UnrequiredField<String>,

    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    #[validate(custom = "validate_rate_limit_patch")]
    pub rate_limit: UnrequiredNullableField<u16>,

    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    #[validate]
    pub uid: UnrequiredNullableField<ApplicationUid>,

    #[serde(default, skip_serializing_if = "UnrequiredField::is_absent")]
    pub metadata: UnrequiredField<Metadata>,
}

impl ModelIn for ApplicationPatch {
    type ActiveModel = (application::ActiveModel, applicationmetadata::ActiveModel);

    fn update_model(self, (app, app_metadata): &mut Self::ActiveModel) {
        let ApplicationPatch {
            name,
            rate_limit,
            uid,
            metadata,
        } = self;

        // `model`'s version of `rate_limit` is an i32, while `self`'s is a u16.
        let rate_limit_map = |x: u16| -> i32 { x.into() };
        let data = metadata;

        patch_field_non_nullable!(app, name);
        patch_field_nullable!(app, rate_limit, rate_limit_map);
        patch_field_nullable!(app, uid);
        patch_field_non_nullable!(app_metadata, data);
    }
}

fn validate_name_length_patch(name: &UnrequiredField<String>) -> Result<(), ValidationError> {
    match name {
        UnrequiredField::Absent => Ok(()),
        UnrequiredField::Some(s) => {
            if s.is_empty() {
                Err(validation_error(
                    Some("length"),
                    Some("Application names must be at least one character"),
                ))
            } else {
                Ok(())
            }
        }
    }
}

fn validate_rate_limit_patch(
    rate_limit: &UnrequiredNullableField<u16>,
) -> Result<(), ValidationError> {
    match rate_limit {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(rate_limit) => {
            if *rate_limit > 0 {
                Ok(())
            } else {
                Err(validation_error(
                    Some("range"),
                    Some("Application rate limits must be at least 1 if set"),
                ))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationOut {
    // FIXME: Do we want to use serde(flatten) or just duplicate the keys?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<ApplicationUid>,
    #[schemars(example = "application_name_example")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    pub id: ApplicationId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Metadata,
}

impl From<(application::Model, applicationmetadata::Model)> for ApplicationOut {
    fn from((app, metadata): (application::Model, applicationmetadata::Model)) -> Self {
        Self {
            uid: app.uid,
            name: app.name,
            rate_limit: app.rate_limit.map(|x| x as u16),
            id: app.id,
            created_at: app.created_at.into(),
            updated_at: app.updated_at.into(),
            metadata: metadata.metadata(),
        }
    }
}

/// List of all the organization's applications.
#[aide_annotate(op_id = "v1.application.list")]
async fn list_applications(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination<ReversibleIterator<ApplicationId>>>,
    permissions::Organization { org_id }: permissions::Organization,
) -> Result<Json<ListResponse<ApplicationOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator;
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let query = apply_pagination(
        application::Entity::secure_find(org_id),
        application::Column::Id,
        limit,
        iterator,
        pagination.order.unwrap_or(Ordering::Ascending),
    );

    let results: Vec<ApplicationOut> = query
        .find_also_related(applicationmetadata::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|(app, metadata)| {
            let metadata =
                metadata.unwrap_or_else(|| applicationmetadata::Model::new(app.id.clone()));
            (app, metadata)
        })
        .map(ApplicationOut::from)
        .collect();

    Ok(Json(ApplicationOut::list_response(
        results,
        limit as usize,
        is_prev,
    )))
}

fn default_as_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct CreateApplicationQueryParams {
    /// Get an existing application, or create a new one if doesn't exist. It's two separate functions in the libs.
    #[serde(default = "default_as_false")]
    get_if_exists: bool,
}

/// Create a new application.
#[aide_annotate(op_id = "v1.application.create")]
async fn create_application(
    State(AppState { ref db, .. }): State<AppState>,
    query: ValidatedQuery<CreateApplicationQueryParams>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
) -> Result<JsonStatusUpsert<ApplicationOut>> {
    if let Some(ref uid) = data.uid {
        if let Some((app, metadata)) =
            application::Model::fetch_with_metadata(db, org_id.clone(), uid.clone().into())
                .await
                .trace()?
        {
            if query.get_if_exists {
                // Technically not updated, but it fits.
                return Ok(JsonStatusUpsert::Updated((app, metadata).into()));
            }
            return Err(HttpError::conflict(
                None,
                Some("An application with that id or uid already exists".into()),
            )
            .into());
        };
    }

    let app = application::ActiveModel::new(org_id.clone());
    let metadata = applicationmetadata::ActiveModel::new(app.id.clone().unwrap(), None);

    let mut model = (app, metadata);
    data.update_model(&mut model);
    let (app, metadata) = model;

    let (app, metadata) = db
        .transaction(|txn| {
            async move {
                let app_result = app.insert(txn).await.map_err(http_error_on_conflict)?;
                let metadata = metadata.upsert_or_delete(txn).await.trace()?;
                Ok((app_result, metadata))
            }
            .boxed()
        })
        .await?;

    Ok(JsonStatusUpsert::Created((app, metadata).into()))
}

/// Get an application.
#[aide_annotate(op_id = "v1.application.get")]
async fn get_application(
    permissions::ApplicationWithMetadata { app, metadata }: permissions::ApplicationWithMetadata,
) -> Result<Json<ApplicationOut>> {
    Ok(Json((app, metadata).into()))
}

/// Update an application.
#[aide_annotate(op_id = "v1.application.update")]
async fn update_application(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationPath { app_id }): Path<ApplicationPath>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
) -> Result<JsonStatusUpsert<ApplicationOut>> {
    let (app, metadata, create_models) = if let Some((app, metadata)) =
        application::Model::fetch_with_metadata(db, org_id.clone(), app_id)
            .await
            .trace()?
    {
        (app.into(), metadata.into(), false)
    } else {
        let app = application::ActiveModel::new(org_id);
        let metadata = applicationmetadata::ActiveModel::new(app.id.clone().unwrap(), None);
        (app, metadata, true)
    };

    let mut models = (app, metadata);
    data.update_model(&mut models);
    let (app, metadata) = models;

    let (app, metadata) = db
        .transaction(|txn| {
            async move {
                let app = if create_models {
                    app.insert(txn).await.map_err(http_error_on_conflict)?
                } else {
                    app.update(txn).await.map_err(http_error_on_conflict)?
                };
                let metadata = metadata.upsert_or_delete(txn).await?;
                Ok((app, metadata))
            }
            .boxed()
        })
        .await?;

    if create_models {
        Ok(JsonStatusUpsert::Created((app, metadata).into()))
    } else {
        Ok(JsonStatusUpsert::Updated((app, metadata).into()))
    }
}

/// Partially update an application.
#[aide_annotate]
async fn patch_application(
    State(AppState { ref db, .. }): State<AppState>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<ApplicationPatch>,
) -> Result<Json<ApplicationOut>> {
    let metadata = app.fetch_or_create_metadata(db).await.trace()?;
    let app: application::ActiveModel = app.into();

    let mut model = (app, metadata);
    data.update_model(&mut model);
    let (app, metadata) = model;

    let (app, metadata) = db
        .transaction(|txn| {
            async move {
                let app = app.update(txn).await.map_err(http_error_on_conflict)?;
                let metadata = metadata.upsert_or_delete(txn).await.trace()?;
                Ok((app, metadata))
            }
            .boxed()
        })
        .await?;

    Ok(Json((app, metadata).into()))
}

/// Delete an application.
#[aide_annotate(op_id = "v1.application.delete")]
async fn delete_application(
    State(AppState { ref db, .. }): State<AppState>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
) -> Result<NoContent> {
    let mut app: application::ActiveModel = app.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
    Ok(NoContent)
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Application");
    ApiRouter::new()
        .api_route_with(
            "/app",
            post_with(create_application, create_application_operation)
                .get_with(list_applications, list_applications_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id",
            get_with(get_application, get_application_operation)
                .put_with(update_application, update_application_operation)
                .patch_with(patch_application, patch_application_operation)
                .delete_with(delete_application, delete_application_operation),
            tag,
        )
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use validator::Validate;

    use super::{ApplicationIn, ApplicationPatch};

    const APP_NAME_INVALID: &str = "";
    const APP_NAME_VALID: &str = "test-app";
    const RATE_LIMIT_INVALID: u16 = 0;
    const RATE_LIMIT_VALID: u16 = 1;
    const UID_INVALID: &str = "$$invalid-uid";
    const UID_VALID: &str = "valid-uid";

    #[test]
    fn test_application_in_validation() {
        let invalid_1: ApplicationIn =
            serde_json::from_value(json!({ "name": APP_NAME_INVALID })).unwrap();
        let invalid_2: ApplicationIn = serde_json::from_value(json!({
                    "name": APP_NAME_VALID,
                    "rateLimit": RATE_LIMIT_INVALID }))
        .unwrap();
        let invalid_3: ApplicationIn = serde_json::from_value(json!({
                    "name": APP_NAME_VALID,
                    "uid": UID_INVALID }))
        .unwrap();

        for a in [invalid_1, invalid_2, invalid_3] {
            assert!(a.validate().is_err());
        }

        let valid: ApplicationIn = serde_json::from_value(json!({
            "name": APP_NAME_VALID,
            "rateLimit": RATE_LIMIT_VALID,
            "uid": UID_VALID,
        }))
        .unwrap();
        valid.validate().unwrap();
    }

    // FIXME: How to eliminate the repetition here?
    #[test]
    fn test_application_patch_validation() {
        let invalid_1: ApplicationPatch =
            serde_json::from_value(json!({ "name": APP_NAME_INVALID })).unwrap();
        let invalid_2: ApplicationPatch = serde_json::from_value(json!({
                    "name": APP_NAME_VALID,
                    "rateLimit": RATE_LIMIT_INVALID }))
        .unwrap();
        let invalid_3: ApplicationPatch = serde_json::from_value(json!({
                    "name": APP_NAME_VALID,
                    "uid": UID_INVALID }))
        .unwrap();

        for a in [invalid_1, invalid_2, invalid_3] {
            assert!(a.validate().is_err());
        }

        let valid: ApplicationPatch = serde_json::from_value(json!({
            "name": APP_NAME_VALID,
            "rateLimit": RATE_LIMIT_VALID,
            "uid": UID_VALID,
        }))
        .unwrap();
        valid.validate().unwrap();
    }
}
