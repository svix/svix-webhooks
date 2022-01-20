// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::types::{ApplicationId, ApplicationIdOrUid, ApplicationUid},
    error::{HttpError, Result},
    v1::utils::{EmptyResponse, ListResponse, ModelIn, ModelOut, ValidatedJson, ValidatedQuery},
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

use crate::core::security::Permissions;
use crate::db::models::application;
use crate::v1::utils::Pagination;

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct ApplicationIn {
    #[validate(length(min = 1))]
    name: String,
    #[validate(range(min = 1))]
    rate_limit: Option<u16>,
    /// Optional unique identifier for the application
    #[validate]
    uid: Option<ApplicationUid>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for ApplicationIn {
    type ActiveModel = application::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.name = Set(self.name);
        model.rate_limit = Set(self.rate_limit.map(|x| x.into()));
        model.uid = Set(self.uid);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ModelOut)]
#[serde(rename_all = "camelCase")]
struct ApplicationOut {
    // FIXME: Do we want to use serde(flatten) or just duplicate the keys?
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<ApplicationUid>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<u16>,

    id: ApplicationId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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

async fn list_applications(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<ApplicationId>>,
    permissions: Permissions,
) -> Result<Json<ListResponse<ApplicationOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = application::Entity::secure_find(permissions.org_id)
        .order_by_asc(application::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(application::Column::Id.gt(iterator))
    }

    Ok(Json(ApplicationOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn create_application(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<ApplicationOut>)> {
    let app = application::ActiveModel {
        org_id: Set(permissions.org_id.clone()),
        ..data.into()
    };
    let ret = app.insert(db).await?;
    Ok((StatusCode::CREATED, Json(ret.into())))
}

async fn get_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    permissions: Permissions,
) -> Result<Json<ApplicationOut>> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id, app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(app.into()))
}

async fn update_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    ValidatedJson(data): ValidatedJson<ApplicationIn>,
    permissions: Permissions,
) -> Result<Json<ApplicationOut>> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id.clone(), app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: application::ActiveModel = app.into();
    data.update_model(&mut app);

    let ret = app.update(db).await?;
    Ok(Json(ret.into()))
}

async fn delete_application(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(app_id): Path<ApplicationIdOrUid>,
    permissions: Permissions,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let app = application::Entity::secure_find_by_id_or_uid(permissions.org_id, app_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: application::ActiveModel = app.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new()
        .route("/app/", get(list_applications))
        .route("/app/", post(create_application))
        .route(
            "/app/:app_id/",
            get(get_application)
                .put(update_application)
                .delete(delete_application),
        )
}
