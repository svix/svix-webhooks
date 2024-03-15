// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use futures::FutureExt;
use hyper::StatusCode;
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, ActiveValue::Set, IntoActiveModel, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use svix_server_derive::{aide_annotate, ModelIn, ModelOut};
use validator::{Validate, ValidationError};

use crate::{
    core::{
        cache::Cache,
        message_app::CreateMessageApp,
        permissions,
        types::{
            EndpointId, EventChannel, EventChannelSet, EventTypeName, EventTypeNameSet,
            MessageAttemptTriggerType, MessageId, MessageUid,
        },
    },
    db::models::{application, message, messagecontent},
    error::{Error, HttpError, Result},
    queue::{MessageTaskBatch, TaskQueueProducer},
    v1::utils::{
        apply_pagination_desc, iterator_from_before_or_after, openapi_tag, validation_error,
        ApplicationMsgPath, EventTypesQueryParams, JsonStatus, ListResponse, ModelIn, ModelOut,
        PaginationDescending, PaginationLimit, ReversibleIterator, ValidatedJson, ValidatedQuery,
    },
    AppState,
};

pub fn validate_channels_msg(channels: &EventChannelSet) -> Result<(), ValidationError> {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawPayload(pub Box<RawValue>);

impl JsonSchema for RawPayload {
    fn schema_name() -> String {
        "RawPayload".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        serde_json::Value::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        false
    }
}

impl std::fmt::Display for RawPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.get())
    }
}

impl Eq for RawPayload {}

impl PartialEq for RawPayload {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl RawPayload {
    pub fn from_string(val: String) -> serde_json::Result<Self> {
        Ok(Self(RawValue::from_string(val)?))
    }
}

pub fn validate_raw_payload_is_object(payload: &RawPayload) -> Result<(), ValidationError> {
    // Verify it's an object/map
    if payload.0.get().starts_with('{') {
        Ok(())
    } else {
        Err(validation_error(
            Some("payload"),
            Some("Payload must be an object."),
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageIn {
    /// Optional unique identifier for the message
    #[validate]
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub uid: Option<MessageUid>,
    #[validate]
    pub event_type: EventTypeName,
    #[validate(custom = "validate_raw_payload_is_object")]
    #[serde(alias = "payload", alias = "data")]
    #[schemars(example = "example_payload")]
    pub payload: RawPayload,
    /// List of free-form identifiers that endpoints can filter by
    #[validate(custom = "validate_channels_msg")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(example = "example_channel_set", length(min = 1, max = 5))]
    pub channels: Option<EventChannelSet>,
    #[validate(range(min = 5, max = 90))]
    #[serde(default = "default_90")]
    #[schemars(example = "default_90")]
    pub payload_retention_period: i64,
}

fn example_channel_set() -> Vec<&'static str> {
    vec!["project_123", "group_2"]
}

fn example_payload() -> serde_json::Value {
    serde_json::json!({
        "email": "test@example.com",
        "username": "test_user"
    })
}

// FIXME: This can and should be a derive macro
impl ModelIn for MessageIn {
    type ActiveModel = message::ActiveModel;

    fn update_model(self, model: &mut message::ActiveModel) {
        let MessageIn {
            uid,
            event_type,
            channels,
            payload_retention_period,
            ..
        } = self;

        let expiration = Utc::now() + Duration::days(payload_retention_period);

        model.uid = Set(uid);
        model.event_type = Set(event_type);
        model.expiration = Set(expiration.with_timezone(&Utc).into());
        model.channels = Set(channels);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageOut {
    /// Optional unique identifier for the message
    #[serde(rename = "eventId")]
    pub uid: Option<MessageUid>,
    pub event_type: EventTypeName,
    #[schemars(example = "example_payload")]
    pub payload: RawPayload,
    /// List of free-form identifiers that endpoints can filter by
    #[schemars(length(min = 1, max = 5), example = "example_channel_set")]
    pub channels: Option<EventChannelSet>,
    pub id: MessageId,
    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

impl MessageOut {
    pub fn from_msg_and_payload(model: message::Model, content: Option<Vec<u8>>) -> Self {
        let payload = content
            .and_then(|p| match serde_json::from_slice(&p) {
                Ok(v) => Some(v),
                Err(e) => {
                    tracing::error!("Failed to parse content: {e}");
                    None
                }
            })
            .or(model.legacy_payload);
        let payload = RawPayload::from_string(match payload {
            Some(payload) => serde_json::to_string(&payload).expect("Can never fail"),
            None => r#"{"expired":true}"#.to_string(),
        })
        .expect("Can never fail");

        Self {
            uid: model.uid,
            event_type: model.event_type,
            payload,
            channels: model.channels,
            id: model.id,
            created_at: model.created_at.into(),
        }
    }

    pub fn without_payload(model: message::Model) -> Self {
        Self {
            uid: model.uid,
            event_type: model.event_type,
            payload: RawPayload::from_string("{}".to_string()).expect("Can never fail"),
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

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
pub struct ListMessagesQueryParams {
    /// Filter response based on the channel
    #[validate]
    channel: Option<EventChannel>,
    /// Only include items created before a certain date
    before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    after: Option<DateTime<Utc>>,
    /// When `true` message payloads are included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

/// List all of the application's messages.
///
/// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
/// The `after` parameter lets you filter all items created after a certain date and is ignored if an iterator is passed.
/// `before` and `after` cannot be used simultaneously.
#[aide_annotate(op_id = "v1.message.list")]
async fn list_messages(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<PaginationDescending<ReversibleIterator<MessageId>>>,
    ValidatedQuery(ListMessagesQueryParams {
        channel,
        with_content,
        before,
        after,
    }): ValidatedQuery<ListMessagesQueryParams>,
    EventTypesQueryParams(event_types): EventTypesQueryParams,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageOut>>> {
    let PaginationLimit(limit) = pagination.limit;

    let mut query = message::Entity::secure_find(app.id);

    if let Some(EventTypeNameSet(event_types)) = event_types {
        query = query.filter(message::Column::EventType.is_in(event_types));
    }

    if let Some(channel) = channel {
        query = query.filter(Expr::cust_with_values("channels @> $1", [channel.jsonb()]));
    }

    let iterator = iterator_from_before_or_after(pagination.iterator, before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let query = apply_pagination_desc(query, message::Column::Id, limit, iterator)
        .find_also_related(messagecontent::Entity);

    let msgs_and_content: Vec<(message::Model, Option<messagecontent::Model>)> =
        query.all(db).await?.into_iter().collect();
    let into = |(msg, content): (message::Model, Option<messagecontent::Model>)| {
        if with_content {
            MessageOut::from_msg_and_payload(msg, content.map(|c| c.payload))
        } else {
            MessageOut::without_payload(msg)
        }
    };

    Ok(Json(MessageOut::list_response(
        msgs_and_content.into_iter().map(into).collect(),
        limit as usize,
        is_prev,
    )))
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct CreateMessageQueryParams {
    /// When `true` message payloads are included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

/// Creates a new message and dispatches it to all of the application's endpoints.
///
/// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
/// If a message with the same `eventId` already exists for any application in your environment, a 409 conflict error will be returned.
///
/// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
/// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
///
/// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
#[aide_annotate(op_id = "v1.message.create")]
async fn create_message(
    State(AppState {
        ref db,
        queue_tx,
        cache,
        ..
    }): State<AppState>,
    ValidatedQuery(CreateMessageQueryParams { with_content }): ValidatedQuery<
        CreateMessageQueryParams,
    >,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<MessageIn>,
) -> Result<JsonStatus<202, MessageOut>> {
    Ok(JsonStatus(
        create_message_inner(db, queue_tx, cache, with_content, None, data, app).await?,
    ))
}

pub(crate) async fn create_message_inner(
    db: &DatabaseConnection,
    queue_tx: TaskQueueProducer,
    cache: Cache,
    with_content: bool,
    force_endpoint: Option<EndpointId>,
    data: MessageIn,
    app: application::Model,
) -> Result<MessageOut> {
    let create_message_app = CreateMessageApp::layered_fetch(
        &cache,
        db,
        Some(app.clone()),
        app.org_id.clone(),
        app.id.clone(),
        std::time::Duration::from_secs(30),
    )
    .await?
    // Should never happen since you're giving it an existing Application, but just in case
    .ok_or_else(|| Error::generic(format!("Application doesn't exist: {}", app.id)))?;

    let payload = data.payload.to_string().into_bytes();
    let msg = message::ActiveModel {
        app_id: Set(app.id.clone()),
        org_id: Set(app.org_id),
        ..data.into()
    };

    let (msg, msg_content) = db
        .transaction(|txn| {
            async move {
                let msg = msg.insert(txn).await?;
                let msg_content = messagecontent::ActiveModel::new(msg.id.clone(), payload);
                let msg_content = msg_content.insert(txn).await?;
                Ok((msg, msg_content))
            }
            .boxed()
        })
        .await?;

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
                    force_endpoint,
                    MessageAttemptTriggerType::Scheduled,
                ),
                None,
            )
            .await?;
    }

    let msg_out = if with_content {
        MessageOut::from_msg_and_payload(msg, Some(msg_content.payload))
    } else {
        MessageOut::without_payload(msg)
    };

    Ok(msg_out)
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct GetMessageQueryParams {
    /// When `true` message payloads are included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

/// Get a message by its ID or eventID.
#[aide_annotate(op_id = "v1.message.get")]
async fn get_message(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    ValidatedQuery(GetMessageQueryParams { with_content }): ValidatedQuery<GetMessageQueryParams>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<MessageOut>> {
    let (msg, msg_content) = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .find_also_related(messagecontent::Entity)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    let msg_out = if with_content {
        MessageOut::from_msg_and_payload(msg, msg_content.map(|c| c.payload))
    } else {
        MessageOut::without_payload(msg)
    };
    Ok(Json(msg_out))
}

/// Delete the given message's payload. Useful in cases when a message was accidentally sent with sensitive content.
///
/// The message can't be replayed or resent once its payload has been deleted or expired.
#[aide_annotate(op_id = "v1.message.expunge-content")]
async fn expunge_message_content(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
) -> Result<StatusCode> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    let msg_id = msg.id.clone();
    let mut msg = msg.into_active_model();

    msg.legacy_payload = Set(None);
    msg.update(db).await?;

    messagecontent::Entity::delete_by_id(msg_id)
        .exec(db)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Message");
    ApiRouter::new()
        .api_route_with(
            "/app/:app_id/msg",
            post_with(create_message, create_message_operation)
                .get_with(list_messages, list_messages_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id",
            get_with(get_message, get_message_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id/content",
            delete_with(expunge_message_content, expunge_message_content_operation),
            tag,
        )
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use validator::Validate;

    use super::{
        default_true, CreateMessageQueryParams, GetMessageQueryParams, ListMessagesQueryParams,
        MessageIn,
    };

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
