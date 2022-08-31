// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::AuthenticatedOrganization,
        types::{EventTypeName, RetrySchedule},
    },
    db::models::eventtype,
    error::{HttpError, Result},
    v1::utils::ValidatedJson,
};
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct RetryScheduleInOut {
    retry_schedule: Option<RetrySchedule>,
}

async fn get_retry_schedule(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    AuthenticatedOrganization { permissions }: AuthenticatedOrganization,
) -> Result<(StatusCode, Json<RetryScheduleInOut>)> {
    let evtype = eventtype::Entity::secure_find_by_name(permissions.org_id, evtype_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))
        .unwrap();

    let retry_schedule = RetryScheduleInOut {
        retry_schedule: evtype.retry_schedule,
    };

    Ok((StatusCode::OK, Json(retry_schedule)))
}

async fn update_retry_schedule(
    Extension(ref db): Extension<DatabaseConnection>,
    Path(evtype_name): Path<EventTypeName>,
    ValidatedJson(data): ValidatedJson<RetryScheduleInOut>,
    AuthenticatedOrganization { permissions }: AuthenticatedOrganization,
) -> Result<(StatusCode, Json<RetryScheduleInOut>)> {
    let evtype = eventtype::Entity::secure_find_by_name(permissions.org_id, evtype_name)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut evtype: eventtype::ActiveModel = evtype.into();
    evtype.retry_schedule = Set(data.retry_schedule);
    let evtype = evtype.update(db).await?;

    let retry_schedule = RetryScheduleInOut {
        retry_schedule: evtype.retry_schedule,
    };

    Ok((StatusCode::OK, Json(retry_schedule)))
}

pub fn router() -> Router {
    Router::new().route(
        "/event-type/:event_type_name/retry-schedule/",
        get(get_retry_schedule).put(update_retry_schedule),
    )
}
