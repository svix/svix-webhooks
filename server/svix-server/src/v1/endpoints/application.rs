// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        permissions,
        types::{metadata::Metadata, ApplicationId, ApplicationIdOrUid, ApplicationUid},
    },
    ctx,
    db::models::{application, applicationmetadata},
    error::{HttpError, Result},
    transaction,
    v1::utils::{
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired,
        validation_error, EmptyResponse, ListResponse, ModelIn, ModelOut, Pagination,
        PaginationLimit, ValidatedJson, ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use svix_server_derive::ModelOut;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationIn {
    #[validate(
        length(min = 1, message = "Application names must be at least one character"),
        custom = "validate_no_control_characters"
    )]
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

#[derive(Deserialize, Serialize, Validate)]
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

fn validate_name_length_patch(
    name: &UnrequiredField<String>,
) -> std::result::Result<(), ValidationError> {
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
) -> std::result::Result<(), ValidationError> {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationOut {
    // FIXME: Do we want to use serde(flatten) or just duplicate the keys?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<ApplicationUid>,
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

async fn list_applications(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<ApplicationId>>,
    permissions::Organization { org_id }: permissions::Organization,
) -> Result<Json<ListResponse<ApplicationOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator.clone();

    let apps =
        ctx!(application::Model::fetch_many_with_metadata(db, org_id, limit + 1, iterator).await)?;

    let results = apps.map(ApplicationOut::from).collect();

    Ok(Json(ApplicationOut::list_response_no_prev(
        results,
        limit as usize,
    )))
}

fn default_as_false() -> bool {
    false
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApplicationQuery {
    #[serde(default = "default_as_false")]
    get_if_exists: bool,
}

async fn create_application(
    Extension(ref db): Extension<DatabaseConnection>,
    query: ValidatedQuery<CreateApplicationQuery>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    if let Some(ref uid) = data.uid {
        if let Some((app, metadata)) = ctx!(
            application::Model::fetch_with_metadata(db, org_id.clone(), uid.clone().into()).await
        )? {
            if query.get_if_exists {
                return Ok((StatusCode::OK, Json((app, metadata).into())));
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

    let (app, metadata) = transaction!(db, |txn| async move {
        let app_result = ctx!(app.insert(txn).await)?;
        let metadata = ctx!(metadata.upsert_or_delete(txn).await)?;
        Ok((app_result, metadata))
    })?;

    Ok((StatusCode::CREATED, Json((app, metadata).into())))
}

async fn get_application(
    permissions::ApplicationWithMetadata { app, metadata }: permissions::ApplicationWithMetadata,
) -> Result<Json<ApplicationOut>> {
    Ok(Json((app, metadata).into()))
}

async fn update_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    permissions::Organization { org_id }: permissions::Organization,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    let (app, metadata, create_models) = if let Some((app, metadata)) =
        ctx!(application::Model::fetch_with_metadata(db, org_id.clone(), app_id).await)?
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

    let (app, metadata) = transaction!(db, |txn| async move {
        let app = if create_models {
            ctx!(app.insert(txn).await)?
        } else {
            ctx!(app.update(txn).await)?
        };
        let metadata = ctx!(metadata.upsert_or_delete(txn).await)?;
        Ok((app, metadata))
    })?;

    let status = if create_models {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json((app, metadata).into())))
}

async fn patch_application(
    Extension(ref db): Extension<DatabaseConnection>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<ApplicationPatch>,
) -> Result<Json<ApplicationOut>> {
    let metadata = ctx!(app.fetch_or_create_metadata(db).await)?;
    let app: application::ActiveModel = app.into();

    let mut model = (app, metadata);
    data.update_model(&mut model);
    let (app, metadata) = model;

    let (app, metadata) = transaction!(db, |txn| async move {
        let app = ctx!(app.update(txn).await)?;
        let metadata = ctx!(metadata.upsert_or_delete(txn).await)?;
        Ok((app, metadata))
    })?;

    Ok(Json((app, metadata).into()))
}

async fn delete_application(
    Extension(ref db): Extension<DatabaseConnection>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let mut app: application::ActiveModel = app.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    ctx!(app.update(db).await)?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new()
        .route("/app/", post(create_application).get(list_applications))
        .route(
            "/app/:app_id/",
            get(get_application)
                .put(update_application)
                .patch(patch_application)
                .delete(delete_application),
        )
}

#[cfg(test)]
mod tests {
    use super::{ApplicationIn, ApplicationPatch};
    use serde_json::json;
    use validator::Validate;

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
