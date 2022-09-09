// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::{
            AuthenticatedApplication, AuthenticatedOrganization,
            AuthenticatedOrganizationWithApplication,
        },
        types::{ApplicationId, ApplicationIdOrUid, ApplicationUid},
    },
    db::models::application,
    error::{HttpError, Result},
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
use anyhow::Context;
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
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate, ModelIn)]
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
}

// FIXME: This can and should be a derive macro
impl ModelIn for ApplicationIn {
    type ActiveModel = application::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let ApplicationIn {
            name,
            rate_limit,
            uid,
        } = self;

        model.name = Set(name);
        model.rate_limit = Set(rate_limit.map(|x| x.into()));
        model.uid = Set(uid);
    }
}

#[derive(Deserialize, ModelIn, Serialize, Validate)]
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
}

impl ModelIn for ApplicationPatch {
    type ActiveModel = application::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let ApplicationPatch {
            name,
            rate_limit,
            uid,
        } = self;

        // `model`'s version of `rate_limit` is an i32, while `self`'s is a u16.
        let rate_limit_map = |x: u16| -> i32 { x.into() };

        patch_field_non_nullable!(model, name);
        patch_field_nullable!(model, rate_limit, rate_limit_map);
        patch_field_nullable!(model, uid);
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
}

// FIXME: This can and should be a derive macro
impl From<application::Model> for ApplicationOut {
    fn from(model: application::Model) -> Self {
        Self {
            uid: model.uid,
            name: model.name,
            rate_limit: model.rate_limit.map(|x| x as u16),

            id: model.id,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

// TODO GABRIEL REMOVE
fn fallible() -> std::result::Result<(), DbErr> {
    Err(DbErr::Custom("some internal server error happened".into()))
}

async fn list_applications(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<ApplicationId>>,
    AuthenticatedOrganization { permissions }: AuthenticatedOrganization,
) -> Result<Json<ListResponse<ApplicationOut>>> {
    fallible().with_context(|| "fallible thing happened")?;

    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = application::Entity::secure_find(permissions.org_id)
        .order_by_asc(application::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(application::Column::Id.gt(iterator))
    }

    Ok(Json(ApplicationOut::list_response_no_prev(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
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
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    query: ValidatedQuery<CreateApplicationQuery>,
    AuthenticatedOrganization { permissions }: AuthenticatedOrganization,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    if query.get_if_exists {
        if let Some(ref uid) = data.uid {
            let app = application::Entity::secure_find(permissions.org_id.clone())
                .filter(application::Column::Uid.eq(uid.to_owned()))
                .one(db)
                .await?;
            if let Some(ret) = app {
                return Ok((StatusCode::OK, Json(ret.into())));
            }
        }
    }

    let app = application::ActiveModel {
        org_id: Set(permissions.org_id.clone()),
        ..data.into()
    };
    let ret = app.insert(db).await?;
    Ok((StatusCode::CREATED, Json(ret.into())))
}

async fn get_application(
    Extension(ref db): Extension<DatabaseConnection>,
    AuthenticatedApplication { app, permissions }: AuthenticatedApplication,
) -> Result<Json<ApplicationOut>> {
    let app = application::Entity::secure_find_by_id(permissions.org_id, app.id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(app.into()))
}

async fn update_application(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    Path(app_id): Path<ApplicationIdOrUid>,
    AuthenticatedOrganization { permissions }: AuthenticatedOrganization,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id.clone(), app_id)
        .one(db)
        .await?;

    match app {
        Some(app) => {
            let mut app: application::ActiveModel = app.into();
            data.update_model(&mut app);
            let ret = app.update(db).await?;

            Ok((StatusCode::OK, Json(ret.into())))
        }
        None => {
            let ret = application::ActiveModel {
                org_id: Set(permissions.org_id.clone()),
                ..data.into()
            }
            .insert(db)
            .await?;

            Ok((StatusCode::CREATED, Json(ret.into())))
        }
    }
}

async fn patch_application(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<ApplicationPatch>,
    AuthenticatedOrganizationWithApplication {
        permissions: _,
        app,
    }: AuthenticatedOrganizationWithApplication,
) -> Result<Json<ApplicationOut>> {
    let mut app: application::ActiveModel = app.into();
    data.update_model(&mut app);

    let ret = app.update(db).await?;
    Ok(Json(ret.into()))
}

async fn delete_application(
    Extension(ref db): Extension<DatabaseConnection>,
    AuthenticatedOrganizationWithApplication {
        permissions: _,
        app,
    }: AuthenticatedOrganizationWithApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let mut app: application::ActiveModel = app.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
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
