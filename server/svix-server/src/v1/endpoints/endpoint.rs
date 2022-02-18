// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, BaseId, EndpointId, EndpointIdOrUid, EndpointUid, EventChannelSet,
            EventTypeName, EventTypeNameSet, ExpiringSigningKey, ExpiringSigningKeys,
            MessageAttemptTriggerType, MessageEndpointId, MessageStatus, OrganizationId,
        },
    },
    db::models::{application, eventtype, messagedestination},
    error::{HttpError, Result},
    queue::{MessageTask, TaskQueueProducer},
    v1::utils::{
        api_not_implemented, EmptyResponse, ListResponse, ModelIn, ModelOut, ValidatedJson,
        ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, collections::HashSet, iter};

use svix_server_derive::{ModelIn, ModelOut};
use validator::{Validate, ValidationError};

use crate::core::types::{EndpointHeaders, EndpointSecret};
use crate::db::models::endpoint;
use crate::v1::utils::Pagination;

use self::hack::EventTypeNameResult;

pub fn validate_event_types_ids(
    event_types_ids: &EventTypeNameSet,
) -> std::result::Result<(), ValidationError> {
    if event_types_ids.0.is_empty() {
        Err(ValidationError::new(
            "filterTypes can't be empty, it must have at least one item.",
        ))
    } else {
        Ok(())
    }
}

pub fn validate_channels_endpoint(
    channels: &EventChannelSet,
) -> std::result::Result<(), ValidationError> {
    let len = channels.0.len();
    if !(1..=10).contains(&len) {
        Err(ValidationError::new(
            "Channels must have at least 1 and at most 10 items, or be set to null.",
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct EndpointIn {
    #[serde(default)]
    description: String,
    #[validate(range(min = 1))]
    rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    #[validate]
    uid: Option<EndpointUid>,
    #[validate(url)]
    url: String,
    #[validate(range(min = 1))]
    version: u16,
    #[serde(default)]
    disabled: bool,
    #[serde(rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids")]
    #[validate]
    event_types_ids: Option<EventTypeNameSet>,
    #[validate(custom = "validate_channels_endpoint")]
    #[validate]
    channels: Option<EventChannelSet>,

    #[serde(default)]
    #[serde(rename = "secret")]
    key: Option<EndpointSecret>,
}

// FIXME: This can and should be a derive macro
impl ModelIn for EndpointIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.description = Set(self.description);
        model.rate_limit = Set(self.rate_limit.map(|x| x.into()));
        model.uid = Set(self.uid);
        model.url = Set(self.url);
        model.version = Set(self.version.into());
        model.disabled = Set(self.disabled);
        model.event_types_ids = Set(self.event_types_ids);
        model.channels = Set(self.channels);
        if let Some(key) = self.key {
            model.key = Set(key);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ModelOut)]
#[serde(rename_all = "camelCase")]
struct EndpointOut {
    description: String,
    rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    uid: Option<EndpointUid>,
    url: String,
    version: u16,
    disabled: bool,
    #[serde(rename = "filterTypes")]
    event_types_ids: Option<EventTypeNameSet>,
    channels: Option<EventChannelSet>,

    id: EndpointId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<endpoint::Model> for EndpointOut {
    fn from(model: endpoint::Model) -> Self {
        Self {
            description: model.description,
            rate_limit: model.rate_limit.map(|x| x as u16),
            uid: model.uid,
            url: model.url,
            version: model.version as u16,
            disabled: model.disabled,
            event_types_ids: model.event_types_ids,
            channels: model.channels,

            id: model.id,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EndpointSecretRotateIn {
    #[serde(default)]
    key: Option<EndpointSecret>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct EndpointSecretOut {
    key: EndpointSecret,
}

#[derive(Clone, Debug, PartialEq, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoverIn {
    since: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EndpointHeadersIn {
    #[validate]
    headers: EndpointHeaders,
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct EndpointHeadersOut {
    headers: HashMap<String, String>,
    sensitive: HashSet<String>,
}

impl EndpointHeadersOut {
    const SENSITIVE_HEADERS: &'static [&'static str] = &[
        "x-auth-token",
        "www-authenticate",
        "authorization",
        "proxy-authenticate",
        "proxy-authorization",
    ];
}

impl From<EndpointHeaders> for EndpointHeadersOut {
    fn from(hdr: EndpointHeaders) -> Self {
        let (sens, remaining) = hdr
            .0
            .into_iter()
            .partition(|(k, _)| Self::SENSITIVE_HEADERS.iter().any(|&x| x == k));

        Self {
            headers: remaining,
            sensitive: sens.into_iter().map(|(k, _)| k).collect(),
        }
    }
}

impl ModelIn for EndpointHeadersIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.headers = Set(Some(self.headers));
    }
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
        Err(HttpError::unprocessable_entity(
            Some("value_error".to_owned()),
            Some(format!(
                "The following type names don't exist: {:?}",
                missing
            )),
        )
        .into())
    }
}

async fn list_endpoints(
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

    Ok(Json(EndpointOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn create_endpoint(
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

async fn get_endpoint(
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

async fn update_endpoint(
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

async fn delete_endpoint(
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

async fn get_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<EndpointSecretOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(EndpointSecretOut { key: endp.key }))
}

async fn rotate_endpoint_secret(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointSecretRotateIn>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let mut endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let now = Utc::now();
    let last_key = ExpiringSigningKey {
        key: endp.key.clone(),
        expiration: now + Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
    };

    if let Some(ref old_keys) = endp.old_keys {
        if old_keys.0.len() + 1 > ExpiringSigningKeys::MAX_OLD_KEYS {
            return Err(HttpError::bad_request(
                Some("limit_reached".to_owned()),
                Some(format!(
                    "You can only rotate a key {} times within the last {}.",
                    ExpiringSigningKeys::MAX_OLD_KEYS,
                    ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS
                )),
            )
            .into());
        }
    }

    let old_keys = endp.old_keys.take();

    let endp = endpoint::ActiveModel {
        key: Set(if let Some(key) = data.key {
            key
        } else {
            EndpointSecret::generate()?
        }),

        old_keys: Set(Some(ExpiringSigningKeys(
            iter::once(last_key)
                .chain(
                    old_keys
                        .map(|x| x.0.into_iter())
                        .unwrap_or_else(|| vec![].into_iter()),
                )
                .collect(),
        ))),
        ..endp.into()
    };
    endp.update(db).await?;

    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

async fn bulk_recover_failed_messages(
    db: DatabaseConnection,
    queue_tx: TaskQueueProducer,
    app: application::Model,
    endp: endpoint::Model,
    since: DateTime<Utc>,
) -> Result<()> {
    const RECOVER_LIMIT: u64 = 10_000;
    const BATCH_SIZE: u64 = 100;
    let mut iterator: Option<MessageEndpointId> = None;
    let mut num_done: u64 = 0;

    loop {
        let mut query = messagedestination::Entity::secure_find_by_endpoint(endp.id.clone())
            .filter(messagedestination::Column::Id.gte(MessageEndpointId::start_id(since)))
            .filter(messagedestination::Column::Status.eq(MessageStatus::Fail))
            .order_by_asc(messagedestination::Column::Id)
            .limit(RECOVER_LIMIT);

        if let Some(iterator) = iterator {
            query = query.filter(messagedestination::Column::Id.gt(iterator))
        }

        let items = query.all(&db).await?;
        let cur_len = items.len() as u64;
        iterator = items.last().map(|x| x.id.clone());

        for msg_dest in items {
            queue_tx
                .send(
                    MessageTask::new_task(
                        msg_dest.msg_id,
                        app.id.clone(),
                        msg_dest.endp_id,
                        MessageAttemptTriggerType::Manual,
                    ),
                    None,
                )
                .await?;
        }

        num_done += cur_len;
        if cur_len < BATCH_SIZE || num_done > RECOVER_LIMIT {
            break;
        }
    }

    Ok(())
}

async fn recover_failed_webhooks(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<RecoverIn>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    // Add five minutes so that people can easily just do `now() - two_weeks` without having to worry about clock sync
    let timeframe = chrono::Duration::days(14);
    let timeframe = timeframe + chrono::Duration::minutes(5);

    if data.since < Utc::now() - timeframe {
        return Err(HttpError::unprocessable_entity(
            Some("value_error".to_owned()),
            Some("Cannot recover messages more than 14 days old.".to_owned()),
        )
        .into());
    }

    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let db = db.clone();
    let queue_tx = queue_tx.clone();
    tokio::spawn(
        async move { bulk_recover_failed_messages(db, queue_tx, app, endp, data.since).await },
    );

    Ok((StatusCode::ACCEPTED, Json(EmptyResponse {})))
}

async fn get_endpoint_headers(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<EndpointHeadersOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    match endp.headers {
        Some(h) => Ok(Json(h.into())),
        None => Ok(Json(EndpointHeadersOut::default())),
    }
}

async fn update_endpoint_headers(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointHeadersIn>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut endp: endpoint::ActiveModel = endp.into();
    data.update_model(&mut endp);
    endp.update(db).await?;

    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new().nest(
        "/app/:app_id",
        Router::new()
            .route("/endpoint/", get(list_endpoints))
            .route("/endpoint/", post(create_endpoint))
            .route(
                "/endpoint/:endp_id/",
                get(get_endpoint)
                    .put(update_endpoint)
                    .delete(delete_endpoint),
            )
            .route("/endpoint/:endp_id/secret/", get(get_endpoint_secret))
            .route(
                "/endpoint/:endp_id/secret/rotate/",
                post(rotate_endpoint_secret),
            )
            .route("/endpoint/:endp_id/stats/", get(api_not_implemented))
            .route(
                "/endpoint/:endp_id/send-example/",
                post(api_not_implemented),
            )
            .route("/endpoint/:endp_id/recover/", post(recover_failed_webhooks))
            .route(
                "/endpoint/:endp_id/headers/",
                get(get_endpoint_headers)
                    .patch(api_not_implemented)
                    .put(update_endpoint_headers),
            ),
    )
}

#[cfg(test)]
mod tests {
    use super::EndpointHeadersOut;
    use crate::core::types::EndpointHeaders;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_into_endpoint_headers_out() {
        let ep = EndpointHeaders(HashMap::from([
            ("x-non-sensitive".to_owned(), "all-clear".to_owned()),
            ("authorization".to_owned(), "should-be-omitted".to_owned()),
        ]));

        let epo: EndpointHeadersOut = ep.into();
        assert_eq!(
            HashMap::from([("x-non-sensitive".to_owned(), "all-clear".to_owned())]),
            epo.headers
        );
        assert_eq!(HashSet::from(["authorization".to_owned()]), epo.sensitive);
    }
}
