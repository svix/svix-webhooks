// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aide::axum::{
    routing::{delete_with, get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, IntoActiveModel, QueryOrder, QuerySelect};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;
use svix_server_derive::{aide_annotate, ModelOut};
use validator::Validate;

use crate::{
    core::{
        permissions,
        types::{
            EndpointId, EndpointIdOrUid, EventChannel, EventTypeNameSet, MessageAttemptId,
            MessageAttemptTriggerType, MessageEndpointId, MessageId, MessageStatus,
            StatusCodeClass,
        },
    },
    db::models::{endpoint, message, messageattempt, messagecontent, messagedestination},
    error::{Error, HttpError, Result},
    queue::MessageTask,
    v1::{
        endpoints::message::MessageOut,
        utils::{
            apply_pagination_desc, iterator_from_before_or_after, openapi_tag,
            ApplicationEndpointPath, ApplicationMsgAttemptPath, ApplicationMsgEndpointPath,
            ApplicationMsgPath, EventTypesQueryParams, ListResponse, ModelOut, NoContentWithCode,
            PaginationDescending, PaginationLimit, ReversibleIterator, ValidatedQuery,
        },
    },
    AppState,
};

fn example_status_code() -> i16 {
    200
}

fn example_endpoint_url() -> &'static str {
    "https://example.com/webhook/"
}

fn example_attempt_response() -> &'static str {
    "{}"
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttemptOut {
    #[schemars(url, length(min = 1, max = 65_536), example = "example_endpoint_url")]
    pub url: String,
    #[schemars(example = "example_attempt_response")]
    pub response: String,
    #[schemars(example = "example_status_code")]
    pub response_status_code: i16,
    pub status: MessageStatus,
    pub trigger_type: MessageAttemptTriggerType,
    pub msg_id: MessageId,
    pub endpoint_id: EndpointId,

    pub id: MessageAttemptId,

    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

// FIXME: This can and should be a derive macro
impl From<messageattempt::Model> for MessageAttemptOut {
    fn from(model: messageattempt::Model) -> Self {
        Self {
            url: model.url,
            response: model.response,
            response_status_code: model.response_status_code,
            status: model.status,
            trigger_type: model.trigger_type,
            msg_id: model.msg_id,
            endpoint_id: model.endp_id,

            id: model.id,
            created_at: model.created_at.into(),
        }
    }
}

/// A model containing information on a given message plus additional fields on the last attempt for
/// that message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointMessageOut {
    #[serde(flatten)]
    pub msg: MessageOut,
    pub status: MessageStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<DateTimeWithTimeZone>,
}

impl ModelOut for EndpointMessageOut {
    fn id_copy(&self) -> String {
        self.msg.id.0.clone()
    }
}

impl EndpointMessageOut {
    pub fn from_dest_and_msg(
        dest: messagedestination::Model,
        msg: message::Model,
        with_content: bool,
        msg_payload: Option<Vec<u8>>,
    ) -> EndpointMessageOut {
        EndpointMessageOut {
            msg: if with_content {
                MessageOut::from_msg_and_payload(msg, msg_payload)
            } else {
                MessageOut::without_payload(msg)
            },
            status: dest.status,
            next_attempt: dest.next_attempt,
        }
    }
}

// XXX: only used in tests, so OK if it's a bit hacky
impl<'de> Deserialize<'de> for EndpointMessageOut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value = Box::<RawValue>::deserialize(deserializer)?;
        let rest: HashMap<&str, &RawValue> = serde_json::from_str(raw_value.get()).unwrap();
        Ok(Self {
            msg: serde_json::from_str(raw_value.get()).unwrap(),
            status: serde_json::from_str(rest.get("status").unwrap().get()).unwrap(),
            next_attempt: rest
                .get("next_attempt")
                .map(|x| serde_json::from_str(x.get()).unwrap()),
        })
    }
}

/// Additional parameters (besides pagination) in the query string for the "List Attempted Messages"
/// endpoint.
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ListAttemptedMessagesQueryParams {
    /// Filter response based on the channel
    #[validate]
    channel: Option<EventChannel>,
    /// Filter response based on the delivery status
    status: Option<MessageStatus>,
    /// Only include items created before a certain date
    before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    after: Option<DateTime<Utc>>,
    /// When `true` message payloads are included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

fn default_true() -> bool {
    true
}

/// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
///
/// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
#[aide_annotate(op_id = "v1.message-attempt.list-attempted-messages")]
async fn list_attempted_messages(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<PaginationDescending<ReversibleIterator<MessageId>>>,
    ValidatedQuery(ListAttemptedMessagesQueryParams {
        channel,
        status,
        before,
        after,
        with_content,
    }): ValidatedQuery<ListAttemptedMessagesQueryParams>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    EventTypesQueryParams(event_types): EventTypesQueryParams,
) -> Result<Json<ListResponse<EndpointMessageOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut dests_and_msgs = messagedestination::Entity::secure_find_by_endpoint(endp.id)
        .find_also_related(message::Entity);

    if let Some(channel) = channel {
        dests_and_msgs =
            dests_and_msgs.filter(Expr::cust_with_values("channels @> $1", [channel.jsonb()]));
    }

    if let Some(status) = status {
        dests_and_msgs = dests_and_msgs.filter(messagedestination::Column::Status.eq(status));
    }

    if let Some(EventTypeNameSet(event_types)) = event_types {
        dests_and_msgs = dests_and_msgs.filter(message::Column::EventType.is_in(event_types));
    }

    async fn _get_msg_dest_id(
        db: &DatabaseConnection,
        msg_id: MessageId,
    ) -> Result<MessageEndpointId> {
        Ok(messagedestination::Entity::secure_find_by_msg(msg_id)
            .one(db)
            .await?
            .ok_or_else(|| HttpError::bad_request(None, Some("Invalid iterator".to_owned())))?
            .id)
    }

    let msg_dest_iterator = match pagination.iterator {
        Some(ReversibleIterator::Normal(msg_id)) => Some(ReversibleIterator::Normal(
            _get_msg_dest_id(db, msg_id).await?,
        )),
        Some(ReversibleIterator::Prev(msg_id)) => Some(ReversibleIterator::Prev(
            _get_msg_dest_id(db, msg_id).await?,
        )),
        None => None,
    };
    let iterator = iterator_from_before_or_after(msg_dest_iterator, before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));

    let dests_and_msgs = apply_pagination_desc(
        dests_and_msgs,
        messagedestination::Column::Id,
        limit,
        iterator,
    );

    let dests_and_msgs = dests_and_msgs.all(db).await?;

    let msg_ids = dests_and_msgs
        .iter()
        .filter_map(|(_, msg)| msg.as_ref())
        .map(|msg| msg.id.clone())
        .collect::<Vec<MessageId>>();

    // `find_also_related` can't be used multiple times in a query, so rather
    // than build a complicated custom query, just query all the content
    // data separately. Hopefully this isn't too painful:
    let mut msg_content_map = messagecontent::Entity::secure_find_by_id_in(msg_ids)
        .all(db)
        .await?
        .into_iter()
        .map(|content| (content.id, content.payload))
        .collect::<HashMap<MessageId, Vec<u8>>>();

    let into = |(dest, msg): (messagedestination::Model, Option<message::Model>)| {
        let msg =
            msg.ok_or_else(|| Error::database("No associated message with messagedestination"))?;
        let payload = msg_content_map.remove(&msg.id);
        Ok(EndpointMessageOut::from_dest_and_msg(
            dest,
            msg,
            with_content,
            payload,
        ))
    };

    let out = dests_and_msgs
        .into_iter()
        .map(into)
        .collect::<Result<_>>()?;

    Ok(Json(EndpointMessageOut::list_response(
        out,
        limit as usize,
        is_prev,
    )))
}

/// Additional parameters (besides pagination) in the query string for the "List Attempts by
/// Endpoint" endpoint.
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ListAttemptsByEndpointQueryParams {
    /// Filter response based on the delivery status
    status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[validate]
    channel: Option<EventChannel>,
    /// Only include items created before a certain date
    before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    after: Option<DateTime<Utc>>,
    /// When `true` attempt content is included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

// Applies filters common to [`list_attempts_by_endpoint`] and [`list_attempts_by_msg`]
fn list_attempts_by_endpoint_or_message_filters(
    mut query: Select<messageattempt::Entity>,
    status: Option<MessageStatus>,
    status_code_class: Option<StatusCodeClass>,
    event_types: Option<EventTypeNameSet>,
    channel: Option<EventChannel>,
) -> Select<messageattempt::Entity> {
    if let Some(status) = status {
        query = query.filter(messageattempt::Column::Status.eq(status));
    }

    query = match status_code_class {
        Some(StatusCodeClass::CodeNone) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(0, 99))
        }

        Some(StatusCodeClass::Code1xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(100, 199))
        }

        Some(StatusCodeClass::Code2xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(200, 299))
        }

        Some(StatusCodeClass::Code3xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(300, 399))
        }

        Some(StatusCodeClass::Code4xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(400, 499))
        }

        Some(StatusCodeClass::Code5xx) => {
            query.filter(messageattempt::Column::ResponseStatusCode.between(500, 599))
        }

        None => query,
    };

    // The event_types and channel filter require joining the associated message
    if event_types.is_some() || channel.is_some() {
        query = query.join_rev(
            sea_orm::JoinType::InnerJoin,
            message::Entity::belongs_to(messageattempt::Entity)
                .from(message::Column::Id)
                .to(messageattempt::Column::MsgId)
                .into(),
        );

        if let Some(EventTypeNameSet(event_types)) = event_types {
            query = query.filter(message::Column::EventType.is_in(event_types));
        }

        if let Some(channel) = channel {
            // sea_orm evaluates the '$1' relative to the # of params in `Expr::cust_with_values`,
            // NOT relative to the total number of params in the final query like you might expect.
            // As such, this won't break if more $N params are added in earler/later
            // `.filter` calls.
            query = query.filter(Expr::cust_with_values("channels @> $1", [channel.jsonb()]));
        }
    }

    query
}

/// List attempts by endpoint id
#[aide_annotate(op_id = "v1.message-attempt.list-by-endpoint")]
async fn list_attempts_by_endpoint(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<
        PaginationDescending<ReversibleIterator<MessageAttemptId>>,
    >,
    ValidatedQuery(ListAttemptsByEndpointQueryParams {
        status,
        status_code_class,
        channel,
        before,
        after,
        with_content,
    }): ValidatedQuery<ListAttemptsByEndpointQueryParams>,
    EventTypesQueryParams(event_types): EventTypesQueryParams,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    // Confirm endpoint ID belongs to the given application
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let query = list_attempts_by_endpoint_or_message_filters(
        messageattempt::Entity::secure_find_by_endpoint(endp.id),
        status,
        status_code_class,
        event_types,
        channel,
    );

    let iterator = iterator_from_before_or_after(pagination.iterator, before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));
    let query = apply_pagination_desc(query, messageattempt::Column::Id, limit, iterator);

    let out = query
        .all(db)
        .await?
        .into_iter()
        .map(|mut attempt| {
            if !with_content {
                "{}".clone_into(&mut attempt.response)
            }

            attempt
        })
        .map(Into::into)
        .collect();

    Ok(Json(MessageAttemptOut::list_response(
        out,
        limit as usize,
        is_prev,
    )))
}

/// Flattens in a [`ListAttemptsByEndpointOrMsgQueryParameters`] and adds one extra query parameter
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ListAttemptsByMsgQueryParams {
    /// Filter response based on the delivery status
    status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[validate]
    channel: Option<EventChannel>,
    /// Filter the attempts based on the attempted endpoint
    #[validate]
    endpoint_id: Option<EndpointIdOrUid>,
    /// Only include items created before a certain date
    before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    after: Option<DateTime<Utc>>,
    /// When `true` attempt content is included in the response
    #[serde(default = "default_true")]
    with_content: bool,
}

/// List attempts by message id
#[aide_annotate(op_id = "v1.message-attempt.list-by-msg")]
async fn list_attempts_by_msg(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<
        PaginationDescending<ReversibleIterator<MessageAttemptId>>,
    >,
    ValidatedQuery(ListAttemptsByMsgQueryParams {
        status,
        status_code_class,
        channel,
        endpoint_id,
        before,
        after,
        with_content,
    }): ValidatedQuery<ListAttemptsByMsgQueryParams>,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    EventTypesQueryParams(event_types): EventTypesQueryParams,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    // Confirm message ID belongs to the given application
    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut query = list_attempts_by_endpoint_or_message_filters(
        messageattempt::Entity::secure_find_by_msg(msg.id),
        status,
        status_code_class,
        event_types,
        channel,
    );

    if let Some(endpoint_id) = endpoint_id {
        // Ensure the endpoint ID/UID belongs to the given application
        if let Some(endp) = endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
            .one(db)
            .await?
        {
            // And filter by its ID incase a UID was used
            query = query.filter(messageattempt::Column::EndpId.eq(endp.id));
        } else {
            return Err(Error::http(HttpError::not_found(None, None)));
        }
    }

    let iterator = iterator_from_before_or_after(pagination.iterator, before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));
    let query = apply_pagination_desc(query, messageattempt::Column::Id, limit, iterator);
    let out = query
        .all(db)
        .await?
        .into_iter()
        .map(|mut attempt| {
            if !with_content {
                "{}".clone_into(&mut attempt.response)
            }

            attempt
        })
        .map(Into::into)
        .collect();

    Ok(Json(MessageAttemptOut::list_response(
        out,
        limit as usize,
        is_prev,
    )))
}

// A type combining information from [`messagedestination::Model`]s and [`endpoint::Model`]s to
// output information on attempted destinations
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageEndpointOut {
    #[serde(flatten)]
    endpoint: super::endpoint::EndpointOutCommon,
    id: EndpointId,
    status: MessageStatus,
    next_attempt: Option<DateTime<Utc>>,
}

impl ModelOut for MessageEndpointOut {
    fn id_copy(&self) -> String {
        self.id.0.clone()
    }
}

impl MessageEndpointOut {
    fn from_dest_and_endp(dest: messagedestination::Model, endp: endpoint::Model) -> Self {
        MessageEndpointOut {
            id: endp.id.clone(),
            endpoint: endp.into(),
            status: dest.status,
            next_attempt: dest.next_attempt.map(Into::into),
        }
    }
}

/// `msg_id`: Use a message id or a message `eventId`
#[aide_annotate(op_id = "v1.message-attempt.list-attempted-destinations")]
async fn list_attempted_destinations(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(mut pagination): ValidatedQuery<PaginationDescending<EndpointId>>,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageEndpointOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator = pagination.iterator.take();

    // Confirm message ID belongs to the given application while fetching the ID in case a UID was
    // given
    let msg_id = if let Some(message) =
        message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id.clone())
            .one(db)
            .await?
    {
        message.id
    } else {
        return Err(Error::http(HttpError::not_found(None, None)));
    };

    // Fetch the [`messagedestination::Model`] and associated [`endpoint::Model`]
    let mut query = messagedestination::Entity::secure_find_by_msg(msg_id)
        .find_also_related(endpoint::Entity)
        .order_by_desc(messagedestination::Column::EndpId)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(messagedestination::Column::EndpId.lt(iterator));
    }

    Ok(Json(MessageEndpointOut::list_response_no_prev(
        query
            .all(db)
            .await?
            .into_iter()
            .map(
                |(dest, endp): (messagedestination::Model, Option<endpoint::Model>)| {
                    let endp = endp.ok_or_else(|| {
                        Error::database("No associated endpoint with messagedestination")
                    })?;
                    Ok(MessageEndpointOut::from_dest_and_endp(dest, endp))
                },
            )
            .collect::<Result<_>>()?,
        limit as usize,
    )))
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ListAttemptsForEndpointQueryParams {
    /// Filter response based on the channel
    #[validate]
    pub channel: Option<EventChannel>,
    /// Filter response based on the delivery status
    pub status: Option<MessageStatus>,
    /// Only include items created before a certain date
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    pub after: Option<DateTime<Utc>>,
}

#[derive(Serialize, JsonSchema)]
struct MessageAttemptEndpointOut {
    #[serde(flatten)]
    common_: MessageAttemptOut,
}

impl From<MessageAttemptOut> for MessageAttemptEndpointOut {
    fn from(common_: MessageAttemptOut) -> Self {
        Self { common_ }
    }
}

/// DEPRECATED: please use list_attempts with endpoint_id as a query parameter instead.
///
/// List the message attempts for a particular endpoint.
///
/// Returning the endpoint.
///
/// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
#[aide_annotate(op_id = "v1.message-attempt.list-by-endpoint-deprecated")]
async fn list_attempts_for_endpoint(
    state: State<AppState>,
    pagination: ValidatedQuery<PaginationDescending<ReversibleIterator<MessageAttemptId>>>,
    ValidatedQuery(ListAttemptsForEndpointQueryParams {
        channel,
        status,
        before,
        after,
    }): ValidatedQuery<ListAttemptsForEndpointQueryParams>,
    event_types_query: EventTypesQueryParams,
    Path(ApplicationMsgEndpointPath {
        app_id,
        msg_id,
        endpoint_id,
    }): Path<ApplicationMsgEndpointPath>,
    auth_app: permissions::Application,
) -> Result<Json<ListResponse<MessageAttemptEndpointOut>>> {
    list_messageattempts(
        state,
        pagination,
        ValidatedQuery(AttemptListFetchQueryParams {
            endpoint_id: Some(endpoint_id),
            channel,
            status,
            before,
            after,
        }),
        event_types_query,
        Path(ApplicationMsgPath { app_id, msg_id }),
        auth_app,
    )
    .await
    .map(|Json(list)| {
        let new_data = list
            .data
            .into_iter()
            .map(MessageAttemptEndpointOut::from)
            .collect();
        Json(ListResponse {
            data: new_data,
            done: list.done,
            iterator: list.iterator,
            prev_iterator: list.prev_iterator,
        })
    })
}

#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct AttemptListFetchQueryParams {
    /// Filter the attempts based on the attempted endpoint
    #[validate]
    pub endpoint_id: Option<EndpointIdOrUid>,
    /// Filter response based on the channel
    #[validate]
    pub channel: Option<EventChannel>,
    /// Filter response based on the delivery status
    pub status: Option<MessageStatus>,
    /// Only include items created before a certain date
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    pub after: Option<DateTime<Utc>>,
}

/// Deprecated: Please use "List Attempts by Endpoint" and "List Attempts by Msg" instead.
///
/// `msg_id`: Use a message id or a message `eventId`
#[aide_annotate(op_id = "v1.message-attempt.list-by-msg-deprecated")]
async fn list_messageattempts(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<
        PaginationDescending<ReversibleIterator<MessageAttemptId>>,
    >,
    ValidatedQuery(AttemptListFetchQueryParams {
        endpoint_id,
        channel,
        status,
        before,
        after,
    }): ValidatedQuery<AttemptListFetchQueryParams>,
    EventTypesQueryParams(event_types): EventTypesQueryParams,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut query = messageattempt::Entity::secure_find_by_msg(msg.id);

    if let Some(endpoint_id) = endpoint_id {
        let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
            .one(db)
            .await?
            .ok_or_else(|| HttpError::not_found(None, None))?;
        query = query.filter(messageattempt::Column::EndpId.eq(endp.id))
    }

    if let Some(status) = status {
        query = query.filter(messageattempt::Column::Status.eq(status))
    }

    if let Some(channel) = channel {
        query = query.filter(Expr::cust_with_values("channels @> $1", [channel.jsonb()]));
    }

    if let Some(EventTypeNameSet(event_types)) = event_types {
        query = query.filter(message::Column::EventType.is_in(event_types));
    }

    let iterator = iterator_from_before_or_after(pagination.iterator, before, after);
    let is_prev = matches!(iterator, Some(ReversibleIterator::Prev(_)));
    let query = apply_pagination_desc(query, messageattempt::Column::Id, limit, iterator);
    let out = query.all(db).await?.into_iter().map(Into::into).collect();

    Ok(Json(MessageAttemptOut::list_response(
        out,
        limit as usize,
        is_prev,
    )))
}

/// `msg_id`: Use a message id or a message `eventId`
#[aide_annotate(op_id = "v1.message-attempt.get")]
async fn get_messageattempt(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationMsgAttemptPath {
        msg_id, attempt_id, ..
    }): Path<ApplicationMsgAttemptPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<MessageAttemptOut>> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let attempt = messageattempt::Entity::secure_find_by_msg(msg.id)
        .filter(messageattempt::Column::Id.eq(attempt_id))
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(attempt.into()))
}

/// Resend a message to the specified endpoint.
#[aide_annotate(op_id = "v1.message-attempt.resend")]
async fn resend_webhook(
    State(AppState {
        ref db, queue_tx, ..
    }): State<AppState>,
    Path(ApplicationMsgEndpointPath {
        msg_id,
        endpoint_id,
        ..
    }): Path<ApplicationMsgEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<NoContentWithCode<202>> {
    let (msg, msg_content) = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .find_also_related(messagecontent::Entity)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let msg_content = match msg_content {
        Some(m) => serde_json::from_slice(&m.payload).ok(),
        None => msg.legacy_payload,
    };
    if msg_content.is_none() {
        return Err(HttpError::bad_request(
            Some("missing_payload".to_string()),
            Some("Unable to resend message. Payload is missing (probably expired).".to_string()),
        )
        .into());
    }

    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    // Fetch it to make sure it was even a combination
    let _msg_dest = messagedestination::Entity::secure_find_by_msg(msg.id.clone())
        .filter(messagedestination::Column::EndpId.eq(endp.id.clone()))
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    queue_tx
        .send(
            MessageTask::new_task(
                msg.id.clone(),
                app.id,
                endp.id,
                MessageAttemptTriggerType::Manual,
            ),
            None,
        )
        .await?;
    Ok(NoContentWithCode)
}

/// Deletes the given attempt's response body. Useful when an endpoint accidentally returned sensitive content.
#[aide_annotate(op_id = "v1.message-attempt.expunge-content")]
async fn expunge_attempt_content(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationMsgAttemptPath {
        msg_id, attempt_id, ..
    }): Path<ApplicationMsgAttemptPath>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
) -> Result<StatusCode> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id, msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, Some("Message not found".to_string())))?;

    let mut attempt = messageattempt::Entity::secure_find_by_msg(msg.id)
        .filter(messageattempt::Column::Id.eq(attempt_id))
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, Some("Message attempt not found".to_string())))?
        .into_active_model();

    attempt.response = sea_orm::Set("EXPUNGED".to_string());
    attempt.update(db).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Message Attempt");
    ApiRouter::new()
        // NOTE: [`list_messageattempts`] is deprecated
        .api_route_with(
            "/app/:app_id/msg/:msg_id/attempt",
            get_with(list_messageattempts, list_messageattempts_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id/attempt/:attempt_id",
            get_with(get_messageattempt, get_messageattempt_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id/attempt/:attempt_id/content",
            delete_with(expunge_attempt_content, expunge_attempt_content_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id/endpoint",
            get_with(
                list_attempted_destinations,
                list_attempted_destinations_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/msg/:msg_id/endpoint/:endpoint_id/resend",
            post_with(resend_webhook, resend_webhook_operation),
            &tag,
        )
        // NOTE: [`list_attempts_for_endpoint`] is deprecated
        .api_route_with(
            "/app/:app_id/msg/:msg_id/endpoint/:endpoint_id/attempt",
            get_with(
                list_attempts_for_endpoint,
                list_attempts_for_endpoint_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/msg",
            get_with(list_attempted_messages, list_attempted_messages_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/attempt/endpoint/:endpoint_id",
            get_with(
                list_attempts_by_endpoint,
                list_attempts_by_endpoint_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/attempt/msg/:msg_id",
            get_with(list_attempts_by_msg, list_attempts_by_msg_operation),
            tag,
        )
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use validator::Validate;

    use super::{
        AttemptListFetchQueryParams, ListAttemptedMessagesQueryParams,
        ListAttemptsByEndpointQueryParams, ListAttemptsByMsgQueryParams,
        ListAttemptsForEndpointQueryParams,
    };

    const INVALID_CHANNEL: &str = "$$invalid-channel";
    const VALID_CHANNEL: &str = "valid-channel";
    const INVALID_ENDPOINT_ID: &str = "$$invalid-endpoint";
    const VALID_ENDPOINT_ID: &str = "ep_valid-endpoint";

    #[test]
    fn test_list_attempted_messages_query_params_validation() {
        let q: ListAttemptedMessagesQueryParams =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptedMessagesQueryParams =
            serde_json::from_value(json!({ "channel": VALID_CHANNEL })).unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_list_attempts_by_endpoint_query_parameters_validation() {
        let q: ListAttemptsByEndpointQueryParams =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());
    }

    #[test]
    fn test_list_attempts_by_msg_query_parameters_validation() {
        let q: ListAttemptsByMsgQueryParams =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByMsgQueryParams =
            serde_json::from_value(json!({ "endpoint_id": INVALID_ENDPOINT_ID })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByMsgQueryParams = serde_json::from_value(json!(
            {
                "channel": VALID_CHANNEL,
                "endpoint_id": VALID_ENDPOINT_ID
            }
        ))
        .unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_list_attempts_for_endpoint_query_parameters_validation() {
        let q: ListAttemptsForEndpointQueryParams =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsForEndpointQueryParams =
            serde_json::from_value(json!({ "channel": VALID_CHANNEL })).unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_attempt_list_fetch_options_validation() {
        let q: AttemptListFetchQueryParams =
            serde_json::from_value(json!({ "endpoint_id": INVALID_ENDPOINT_ID })).unwrap();
        assert!(q.validate().is_err());

        let q: AttemptListFetchQueryParams =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: AttemptListFetchQueryParams = serde_json::from_value(json!(
            {
                "endpoint_id": VALID_ENDPOINT_ID,
                "channel": VALID_CHANNEL
            }
        ))
        .unwrap();
        q.validate().unwrap();
    }
}
