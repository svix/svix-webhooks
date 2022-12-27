// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    cache::Cache,
    core::{
        message_app::CreateMessageApp,
        permissions,
        types::{
            ApplicationIdOrUid, EventChannel, EventChannelSet, EventTypeName, EventTypeNameSet,
            MessageAttemptTriggerType, MessageId, MessageIdOrUid, MessageUid,
        },
    },
    ctx, err_generic,
    error::{HttpError, Result},
    queue::{MessageTaskBatch, TaskQueueProducer},
    v1::utils::{
        apply_pagination, iterator_from_before_or_after, validation_error, ListResponse,
        MessageListFetchOptions, ModelIn, ModelOut, PaginationLimit, ReversibleIterator,
        ValidatedJson, ValidatedQuery,
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use sea_orm::entity::prelude::*;
use sea_orm::{sea_query::Expr, ActiveValue::Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use svix_server_derive::{ModelIn, ModelOut};
use validator::{Validate, ValidationError};

use crate::db::models::message;
use crate::v1::utils::Pagination;

pub fn validate_channels_msg(
    channels: &EventChannelSet,
) -> std::result::Result<(), ValidationError> {
    let len = channels.0.len();
    if !(1..=5).contains(&len) {
        Err(validation_error(
            Some("channels"),
            Some("Channels must have at least 1 and at most 5 items, or be set to null."),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_message_in_payload(
    payload: &serde_json::Value,
) -> std::result::Result<(), ValidationError> {
    match payload {
        serde_json::Value::Object(_) => Ok(()),
        _ => Err(validation_error(
            Some("payload"),
            Some("Payload must be an object."),
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
pub struct MessageIn {
    #[validate]
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub uid: Option<MessageUid>,
    #[validate]
    pub event_type: EventTypeName,
    #[validate(custom = "validate_message_in_payload")]
    #[serde(alias = "payload", alias = "data")]
    pub payload: serde_json::Value,
    #[validate(custom = "validate_channels_msg")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<EventChannelSet>,
    #[validate(range(min = 5, max = 90))]
    #[serde(default = "default_90")]
    pub payload_retention_period: i64,
}

// FIXME: This can and should be a derive macro
impl ModelIn for MessageIn {
    type ActiveModel = message::ActiveModel;

    fn update_model(self, model: &mut message::ActiveModel) {
        let MessageIn {
            uid,
            payload,
            event_type,
            channels,
            payload_retention_period,
        } = self;

        let expiration = Utc::now() + Duration::days(payload_retention_period);

        model.uid = Set(uid);
        model.payload = Set(Some(payload));
        model.event_type = Set(event_type);
        model.expiration = Set(expiration.with_timezone(&Utc).into());
        model.channels = Set(channels);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
pub struct MessageOut {
    #[serde(rename = "eventId")]
    pub uid: Option<MessageUid>,
    pub event_type: EventTypeName,
    pub payload: serde_json::Value,
    pub channels: Option<EventChannelSet>,
    pub id: MessageId,
    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

impl MessageOut {
    fn without_payload(model: message::Model) -> Self {
        Self {
            payload: serde_json::json!({}),
            ..model.into()
        }
    }
}

// FIXME: This can and should be a derive macro
impl From<message::Model> for MessageOut {
    fn from(model: message::Model) -> Self {
        Self {
            uid: model.uid,
            event_type: model.event_type,
            payload: match model.payload {
                Some(payload) => payload,
                None => serde_json::json!({ "expired": true }),
            },
            channels: model.channels,
            id: model.id,
            created_at: model.created_at.into(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_90() -> i64 {
    90
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct ListMessagesQueryParams {
    #[validate]
    channel: Option<EventChannel>,
    #[serde(default = "default_true")]
    with_content: bool,

    after: Option<DateTime<Utc>>,
}

async fn list_messages(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(pagination): ValidatedQuery<Pagination<ReversibleIterator<MessageId>>>,
    ValidatedQuery(ListMessagesQueryParams {
        channel,
        with_content,
        after,
    }): ValidatedQuery<ListMessagesQueryParams>,
    list_filter: MessageListFetchOptions,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageOut>>> {
    let PaginationLimit(limit) = pagination.limit;

    let mut query = message::Entity::secure_find(app.id);

    if let Some(EventTypeNameSet(event_types)) = list_filter.event_types {
        query = query.filter(message::Column::EventType.is_in(event_types));
    }

    if let Some(channel) = channel {
        query = query.filter(Expr::cust_with_values("channels ?? ?", vec![channel]));
    }

    let iterator = iterator_from_before_or_after(pagination.iterator, list_filter.before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let query = apply_pagination(query, message::Column::Id, limit, iterator);
    let into = |x: message::Model| {
        if with_content {
            x.into()
        } else {
            MessageOut::without_payload(x)
        }
    };

    let out = if is_prev {
        ctx!(query.all(db).await)?
            .into_iter()
            .rev()
            .map(into)
            .collect()
    } else {
        ctx!(query.all(db).await)?.into_iter().map(into).collect()
    };

    Ok(Json(MessageOut::list_response(out, limit as usize, false)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessageQueryParams {
    #[serde(default = "default_true")]
    with_content: bool,
}

async fn create_message(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Extension(cache): Extension<Cache>,
    ValidatedQuery(CreateMessageQueryParams { with_content }): ValidatedQuery<
        CreateMessageQueryParams,
    >,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<MessageIn>,
) -> Result<(StatusCode, Json<MessageOut>)> {
    let create_message_app = CreateMessageApp::layered_fetch(
        cache,
        db,
        Some(app.clone()),
        app.org_id.clone(),
        app.id.clone(),
        std::time::Duration::from_secs(30),
    )
    .await?
    // Should never happen since you're giving it an existing Application, but just in case
    .ok_or_else(|| err_generic!("Application doesn't exist: {}", app.id))?;

    let msg = message::ActiveModel {
        app_id: Set(app.id.clone()),
        org_id: Set(app.org_id),
        ..data.into()
    };
    let msg = ctx!(msg.insert(db).await)?;

    let trigger_type = MessageAttemptTriggerType::Scheduled;
    if !create_message_app
        .filtered_endpoints(trigger_type, &msg.event_type, msg.channels.as_ref())
        .is_empty()
    {
        queue_tx
            .send(
                MessageTaskBatch::new_task(
                    msg.id.clone(),
                    app.id.clone(),
                    MessageAttemptTriggerType::Scheduled,
                ),
                None,
            )
            .await?;
    }

    let msg_out = if with_content {
        msg.into()
    } else {
        MessageOut::without_payload(msg)
    };

    Ok((StatusCode::ACCEPTED, Json(msg_out)))
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetMessageQueryParams {
    #[serde(default = "default_true")]
    with_content: bool,
}
async fn get_message(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, msg_id)): Path<(ApplicationIdOrUid, MessageIdOrUid)>,
    ValidatedQuery(GetMessageQueryParams { with_content }): ValidatedQuery<GetMessageQueryParams>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<MessageOut>> {
    let msg = ctx!(
        message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?;
    let msg_out = if with_content {
        msg.into()
    } else {
        MessageOut::without_payload(msg)
    };
    Ok(Json(msg_out))
}

pub fn router() -> Router {
    Router::new()
        .route("/app/:app_id/msg/", post(create_message).get(list_messages))
        .route("/app/:app_id/msg/:msg_id/", get(get_message))
}

#[cfg(test)]
mod tests {
    use super::{
        default_true, CreateMessageQueryParams, GetMessageQueryParams, ListMessagesQueryParams,
        MessageIn,
    };
    use serde_json::json;
    use validator::Validate;

    const CHANNEL_INVALID: &str = "$$invalid-channel";
    const CHANNEL_VALID: &str = "valid-channel";
    const EVENT_TYPE_INVALID: &str = "$$invalid-eventType";
    const EVENT_TYPE_VALID: &str = "valid-eventType";
    const EVENT_ID_INVALID: &str = "$$invalid-eventId";
    const EVENT_ID_VALID: &str = "valid-eventId";
    const EVENT_CHANNELS_INVALID: &[&str] = &["valid-event-channel", "&&invalid-event-channel"];
    const EVENT_CHANNELS_VALID: &[&str] = &["valid-event-channel1", "valid-event-channel2"];

    #[test]
    fn test_message_in_validation() {
        let invalid_1: MessageIn = serde_json::from_value(json!({
            "eventId": EVENT_ID_INVALID,
            "eventType": EVENT_TYPE_VALID,
            "payload": {}
        }))
        .unwrap();

        let invalid_2: MessageIn = serde_json::from_value(json!({
            "eventType": EVENT_TYPE_INVALID,
            "payload": {}
        }))
        .unwrap();

        let invalid_3: MessageIn = serde_json::from_value(json!({
            "eventType": EVENT_TYPE_VALID,
            "payload": {},
            "channels": EVENT_CHANNELS_INVALID
        }))
        .unwrap();

        let invalid_4: MessageIn = serde_json::from_value(json!({
            "eventType": EVENT_TYPE_VALID,
            "payload": "this should be invalid",
            "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();

        let invalid_5: MessageIn = serde_json::from_value(json!({
            "eventType": EVENT_TYPE_VALID,
            "payload": json!([ "this should be invalid" ]),
            "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();

        let invalid_6: MessageIn = serde_json::from_value(json!({
            "eventType": EVENT_TYPE_VALID,
            "payload": json!([ { "msg": "this should be invalid" } ]),
            "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();

        for m in [
            invalid_1, invalid_2, invalid_3, invalid_4, invalid_5, invalid_6,
        ] {
            assert!(m.validate().is_err());
        }

        let valid: MessageIn = serde_json::from_value(json!({
            "eventId": EVENT_ID_VALID,
            "eventType": EVENT_TYPE_VALID,
            "payload": {},
            "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();
        valid.validate().unwrap();
    }

    #[test]
    fn test_list_messages_query_params_validation() {
        let invalid: ListMessagesQueryParams =
            serde_json::from_value(json!({ "channel": CHANNEL_INVALID })).unwrap();
        assert!(invalid.validate().is_err());

        let valid: ListMessagesQueryParams =
            serde_json::from_value(json!({ "channel": CHANNEL_VALID })).unwrap();
        valid.validate().unwrap();
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }

    #[test]
    fn test_create_message_query_params_default() {
        let q: CreateMessageQueryParams = serde_json::from_value(json!({})).unwrap();
        assert!(q.with_content);
    }

    #[test]
    fn test_get_message_query_params_default() {
        let q: GetMessageQueryParams = serde_json::from_value(json!({})).unwrap();
        assert!(q.with_content);
    }
}
