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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
struct EndpointIn {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,

    #[validate(range(min = 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<EndpointUid>,
    #[validate(url)]
    url: String,
    #[validate(range(min = 1))]
    version: u16,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    disabled: bool,
    #[serde(rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    event_types_ids: Option<EventTypeNameSet>,
    #[validate(custom = "validate_channels_endpoint")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<EventChannelSet>,

    #[serde(default)]
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ModelOut)]
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

impl ModelIn for EndpointHeadersIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.headers = Set(Some(self.headers));
    }
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

#[derive(Clone, Debug, PartialEq, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EndpointHeadersPatchIn {
    #[validate]
    headers: EndpointHeaders,
}

impl ModelIn for EndpointHeadersPatchIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        model.headers = if let Some(Some(mut hdrs)) = model.headers.take() {
            hdrs.0.extend(self.headers.0);
            Set(Some(hdrs))
        } else {
            Set(Some(self.headers))
        };
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

async fn patch_endpoint_headers(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointHeadersPatchIn>,
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
                    .patch(patch_endpoint_headers)
                    .put(update_endpoint_headers),
            ),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use anyhow::Result;
    use reqwest::StatusCode;
    use sea_orm::ActiveValue::Set;

    use super::{EndpointHeadersOut, EndpointHeadersPatchIn, EndpointIn, EndpointOut};
    use crate::{
        core::types::{EndpointHeaders, EndpointUid},
        db::models::endpoint,
        test_util::{start_svix_server, EmptyResponse, TestClient},
        v1::{
            endpoints::application::tests::{create_test_app, delete_test_app},
            utils::{ListResponse, ModelIn},
        },
    };

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

    #[test]
    fn test_patch_endpoint_model_update() {
        let existing_hdrs = EndpointHeaders(HashMap::from([
            ("x-1".to_owned(), "123".to_owned()),
            ("x-2".to_owned(), "456".to_owned()),
        ]));
        let patched_hdrs = EndpointHeadersPatchIn {
            headers: EndpointHeaders(HashMap::from([
                ("x-1".to_owned(), "789".to_owned()),
                ("x-3".to_owned(), "123".to_owned()),
            ])),
        };
        let updated_hdrs = EndpointHeaders(HashMap::from([
            ("x-1".to_owned(), "789".to_owned()),
            ("x-2".to_owned(), "456".to_owned()),
            ("x-3".to_owned(), "123".to_owned()),
        ]));
        let mut model = endpoint::ActiveModel {
            headers: Set(Some(existing_hdrs)),
            ..Default::default()
        };

        patched_hdrs.update_model(&mut model);
        assert_eq!(model.headers, Set(Some(updated_hdrs)));
    }

    fn endpoint_in(url: &str) -> EndpointIn {
        EndpointIn {
            url: url.to_owned(),
            version: 1,
            ..Default::default()
        }
    }

    async fn post_endpoint_default(
        client: &TestClient,
        app_id: &str,
        ep_url: &str,
    ) -> Result<EndpointOut> {
        client
            .post(
                &format!("api/v1/app/{}/endpoint/", app_id),
                endpoint_in(ep_url),
                StatusCode::CREATED,
            )
            .await
    }

    async fn post_endpoint_in(
        client: &TestClient,
        app_id: &str,
        ep: EndpointIn,
    ) -> Result<EndpointOut> {
        client
            .post(
                &format!("api/v1/app/{}/endpoint/", app_id),
                ep,
                StatusCode::CREATED,
            )
            .await
    }

    async fn get_endpoint(client: &TestClient, app_id: &str, ep_id: &str) -> Result<EndpointOut> {
        client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
                StatusCode::OK,
            )
            .await
    }

    async fn get_404(client: &TestClient, app_id: &str, ep_id: &str) -> Result<()> {
        // Deserialize into a Value because it a basic JSON structure saying "Entity not found"
        let _: serde_json::Value = client
            .get(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
                StatusCode::NOT_FOUND,
            )
            .await?;
        Ok(())
    }

    async fn delete_endpoint(client: &TestClient, app_id: &str, ep_id: &str) -> Result<()> {
        let _: EmptyResponse = client
            .delete(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_id),
                StatusCode::NO_CONTENT,
            )
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_crud() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1: &str = "v1EndpointCrudTestApp1";
        const APP_NAME_2: &str = "v1EndpointCrudTestApp2";

        const EP_URI_APP_1_EP_1_VER_1: &str = "http://v1endpointcrudtestapp1ep1ver1.test";
        const EP_URI_APP_1_EP_1_VER_2: &str = "http://v1EndpointCrudTestApp1Ep1Ver2.test";
        const EP_URI_APP_1_EP_2: &str = "http://v1EndpointCrudTestApp1Ep2.test";
        const EP_URI_APP_2_EP_1: &str = "http://v1EndpointCrudTestApp2Ep1.test";
        const EP_URI_APP_2_EP_2: &str = "http://v1EndpointCrudTestApp2Ep2.test";

        let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap();
        let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap();

        // CREATE
        let app_1_ep_1 = post_endpoint_default(&client, &app_1, EP_URI_APP_1_EP_1_VER_1)
            .await
            .unwrap();
        assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_1);
        assert_eq!(app_1_ep_1.version, 1);

        let app_1_ep_2 = post_endpoint_default(&client, &app_1, EP_URI_APP_1_EP_2)
            .await
            .unwrap();
        assert_eq!(app_1_ep_2.url, EP_URI_APP_1_EP_2);
        assert_eq!(app_1_ep_2.version, 1);

        let app_2_ep_1 = post_endpoint_default(&client, &app_2, EP_URI_APP_2_EP_1)
            .await
            .unwrap();
        assert_eq!(app_2_ep_1.url, EP_URI_APP_2_EP_1);
        assert_eq!(app_2_ep_1.version, 1);

        let app_2_ep_2 = post_endpoint_default(&client, &app_2, EP_URI_APP_2_EP_2)
            .await
            .unwrap();
        assert_eq!(app_2_ep_2.url, EP_URI_APP_2_EP_2);
        assert_eq!(app_2_ep_2.version, 1);

        // READ

        // Can read from correct app
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_1.id).await.unwrap(),
            app_1_ep_1
        );
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_2.id).await.unwrap(),
            app_1_ep_2
        );
        assert_eq!(
            get_endpoint(&client, &app_2, &app_2_ep_1.id).await.unwrap(),
            app_2_ep_1
        );
        assert_eq!(
            get_endpoint(&client, &app_2, &app_2_ep_2.id).await.unwrap(),
            app_2_ep_2
        );

        // Can't read from incorrect app
        get_404(&client, &app_2, &app_1_ep_1.id).await.unwrap();
        get_404(&client, &app_2, &app_1_ep_2.id).await.unwrap();
        get_404(&client, &app_1, &app_2_ep_1.id).await.unwrap();
        get_404(&client, &app_1, &app_2_ep_2.id).await.unwrap();

        // UPDATE
        let app_1_ep_1_id = app_1_ep_1.id;
        let app_1_ep_1: EndpointOut = client
            .put(
                &format!("api/v1/app/{}/endpoint/{}/", app_1, app_1_ep_1_id),
                endpoint_in(EP_URI_APP_1_EP_1_VER_2),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_2);

        // CONFIRM UPDATE
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_1_id).await.unwrap(),
            app_1_ep_1
        );

        // LIST
        let list_app_1: ListResponse<EndpointOut> = client
            .get(&format!("api/v1/app/{}/endpoint/", &app_1), StatusCode::OK)
            .await
            .unwrap();
        assert_eq!(list_app_1.data.len(), 2);
        assert!(list_app_1.data.contains(&app_1_ep_1));
        assert!(list_app_1.data.contains(&app_1_ep_2));

        let list_app_2: ListResponse<EndpointOut> = client
            .get(&format!("api/v1/app/{}/endpoint/", &app_2), StatusCode::OK)
            .await
            .unwrap();
        assert_eq!(list_app_2.data.len(), 2);
        assert!(list_app_2.data.contains(&app_2_ep_1));
        assert!(list_app_2.data.contains(&app_2_ep_2));

        // DELETE
        delete_endpoint(&client, &app_1, &app_1_ep_1.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_1, &app_1_ep_2.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_2, &app_2_ep_1.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_2, &app_2_ep_2.id)
            .await
            .unwrap();

        // CONFIRM DELETION
        get_404(&client, &app_1, &app_1_ep_1.id).await.unwrap();
        get_404(&client, &app_1, &app_1_ep_2.id).await.unwrap();
        get_404(&client, &app_2, &app_2_ep_1.id).await.unwrap();
        get_404(&client, &app_2, &app_2_ep_2.id).await.unwrap();

        delete_test_app(&client, app_1).await.unwrap();
        delete_test_app(&client, app_2).await.unwrap();
    }

    /// Tests that there is at most one endpoint with a single UID for all endpoints associated with
    /// any application
    #[tokio::test]
    async fn test_uid() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1: &str = "v1EndpointUidTestApp1";
        const APP_NAME_2: &str = "v1EndpointUidTestApp2";

        const EP_URI_APP_1_EP_1: &str = "http://v1endpointUidTestApp1Ep1.test";
        const EP_URI_APP_1_EP_2: &str = "http://v1EndpointUidTestApp1Ep2.test";
        const EP_URI_APP_2: &str = "http://v1EndpointUidTestApp2Ep1.test";

        const DUPLICATE_UID: &str = "test_uid";

        // Same App

        // Double Create -- on creation, it should return an error if identical UIDs are used for
        // endpoints in the same app
        let app_id = create_test_app(&client, APP_NAME_1).await.unwrap();
        let uid = EndpointUid(DUPLICATE_UID.to_owned());

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());

        let mut ep_2 = endpoint_in(EP_URI_APP_1_EP_2);
        ep_2.uid = Some(uid.clone());

        let ep_1 = post_endpoint_in(&client, &app_id, ep_1).await.unwrap();
        assert!(post_endpoint_in(&client, &app_id, ep_2).await.is_err());

        // Update One to Existing -- on update it should return an error if attempting to change
        // the UID to that of an existing endpoint associated with the same app
        let ep_2 = post_endpoint_default(&client, &app_id, EP_URI_APP_1_EP_2)
            .await
            .unwrap();

        let mut ep_2_with_invalid_uid = endpoint_in(EP_URI_APP_1_EP_2);
        ep_2_with_invalid_uid.uid = Some(uid.clone());

        assert!(client
            .put::<_, EndpointOut>(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_2.id),
                ep_2_with_invalid_uid,
                StatusCode::OK,
            )
            .await
            .is_err());

        // Update One to Identical -- however it should not return an error if updating the
        // existing endpoint to one with the same UID
        let mut ep_1_with_duplicate_id = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1_with_duplicate_id.uid = Some(uid.clone());

        let ep_1_updated = client
            .put::<_, EndpointOut>(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_1.id),
                ep_1_with_duplicate_id,
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(ep_1.id, ep_1_updated.id);
        assert_eq!(ep_1.uid, ep_1_updated.uid);

        // Delete One then Create One -- UIDs may no be reused after deletion
        delete_endpoint(&client, &app_id, &ep_1.id).await.unwrap();
        delete_endpoint(&client, &app_id, &ep_2.id).await.unwrap();

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());
        assert!(post_endpoint_in(&client, APP_NAME_1, ep_1).await.is_err());

        delete_test_app(&client, app_id).await.unwrap();

        // Different App -- however if they are associated with different applications, identical
        // UIDs are valid
        let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap();
        let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap();

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());

        let mut ep_2 = endpoint_in(EP_URI_APP_2);
        ep_2.uid = Some(uid.clone());

        let ep_1 = post_endpoint_in(&client, &app_1, ep_1).await.unwrap();
        let ep_2 = post_endpoint_in(&client, &app_2, ep_2).await.unwrap();

        delete_endpoint(&client, &app_1, &ep_1.id).await.unwrap();
        delete_endpoint(&client, &app_2, &ep_2.id).await.unwrap();

        delete_test_app(&client, app_1).await.unwrap();
        delete_test_app(&client, app_2).await.unwrap();
    }

    #[test]
    fn test_endpoint_secret_get_and_rotation() {}

    #[test]
    fn test_endpoint_headers_crud() {}
}
