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
use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use schemars::JsonSchema;
use sea_orm::{entity::prelude::*, IntoActiveModel, QueryOrder, QuerySelect, QueryTrait};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;
use svix_server_derive::{aide_annotate, ModelOut};
use validator::Validate;

use crate::{
    core::{
        permissions,
        types::{
            BaseId, EndpointId, EndpointIdOrUid, EventChannel, EventTypeNameSet, MessageAttemptId,
            MessageAttemptTriggerType, MessageId, MessageStatus, MessageStatusText,
            StatusCodeClass,
        },
    },
    db::models::{
        endpoint, message,
        messageattempt::{self, Query},
        messagecontent,
    },
    error::{Error, HttpError, Result},
    queue::MessageTask,
    v1::{
        endpoints::message::MessageOut,
        utils::{
            filter_and_paginate_time_limited, openapi_tag, ApplicationEndpointPath,
            ApplicationMsgAttemptPath, ApplicationMsgEndpointPath, ApplicationMsgPath,
            EventTypesQueryParams, IteratorDirection, ListResponse, ModelOut, NoContentWithCode,
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
    /// Response duration in milliseconds.
    pub response_duration_ms: i64,
    pub status: MessageStatus,
    pub status_text: MessageStatusText,
    pub trigger_type: MessageAttemptTriggerType,
    pub msg_id: MessageId,
    pub endpoint_id: EndpointId,

    pub id: MessageAttemptId,

    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,
}

impl From<messageattempt::Model> for MessageAttemptOut {
    fn from(model: messageattempt::Model) -> Self {
        Self {
            url: model.url,
            response: model.response,
            response_status_code: model.response_status_code,
            response_duration_ms: model.response_duration_ms,
            status: model.status,
            status_text: model.status.into(),
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
    pub fn from_attempt_and_msg(
        attempt: messageattempt::Model,
        msg: message::Model,
        msg_content: Option<Vec<u8>>,
        with_content: bool,
    ) -> EndpointMessageOut {
        let status = if attempt.next_attempt.is_some() {
            MessageStatus::Sending
        } else {
            attempt.status
        };
        EndpointMessageOut {
            msg: MessageOut::from_msg_and_payload(msg, msg_content, with_content),
            status,
            next_attempt: attempt.next_attempt,
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

pub const FUTURE_QUERY_LIMIT: chrono::Duration = chrono::Duration::hours(1);
pub const LIMITED_QUERY_DURATION: chrono::Duration = chrono::Duration::days(90);

pub fn limit_messageattempt_join<Q: QuerySelect + QueryOrder + QueryFilter>(
    mut query: Q,
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
) -> Q {
    const SORT_COLUMN: messageattempt::Column = messageattempt::Column::Id;
    if let Some(before) = before {
        query = query
            .filter(SORT_COLUMN.gt(MessageAttemptId::start_id(before - LIMITED_QUERY_DURATION)));
        query = query
            .filter(SORT_COLUMN.lt(MessageAttemptId::start_id(before + LIMITED_QUERY_DURATION)));
    } else {
        query =
            query.filter(SORT_COLUMN.gt(MessageAttemptId::start_id(now - LIMITED_QUERY_DURATION)));
    }
    if let Some(after) = after {
        query = query.filter(SORT_COLUMN.gt(MessageAttemptId::start_id(after)));
        query =
            query.filter(SORT_COLUMN.lt(MessageAttemptId::end_id(after + LIMITED_QUERY_DURATION)));
    }

    // blanket limit on future
    query.filter(SORT_COLUMN.lt(MessageAttemptId::start_id(now + FUTURE_QUERY_LIMIT)))
}

fn limit_message_join<Q: QuerySelect + QueryOrder + QueryFilter>(
    mut query: Q,
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
) -> Q {
    const SORT_COLUMN: message::Column = message::Column::Id;
    if let Some(before) = before {
        query = query
            .filter(SORT_COLUMN.gt(MessageId::start_id(before - LIMITED_QUERY_DURATION)))
            .filter(SORT_COLUMN.lt(MessageId::end_id(before)));
    } else {
        query = query.filter(SORT_COLUMN.gt(MessageId::start_id(now - LIMITED_QUERY_DURATION)));
    }
    if let Some(after) = after {
        query = query
            .filter(SORT_COLUMN.lt(MessageId::end_id(after + LIMITED_QUERY_DURATION)))
            .filter(SORT_COLUMN.gt(MessageId::start_id(after)));
    }

    // blanket limit on future
    query.filter(SORT_COLUMN.lt(MessageId::start_id(now + FUTURE_QUERY_LIMIT)))
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

    // Get only a single attempt per message. Later, we'll do a separate query to retrieve
    // just the latest attempt for each id in our (small) list of message ids
    let mut msgs_and_dests = message::Entity::secure_find(endp.app_id.clone())
        .inner_join(messageattempt::Entity)
        .filter(messageattempt::Column::EndpId.eq(endp.id.clone()))
        .distinct_on([message::Column::Id.as_column_ref()]);

    if let Some(channel) = channel {
        msgs_and_dests =
            msgs_and_dests.filter(Expr::cust_with_values("channels @> $1", [channel.jsonb()]));
    }

    let now = chrono::Utc::now();

    fn apply_status_filter<E: EntityTrait>(
        query: Select<E>,
        endp_id: EndpointId,
        status: Option<MessageStatus>,
        before: Option<DateTime<Utc>>,
        after: Option<DateTime<Utc>>,
        now: DateTime<Utc>,
    ) -> Select<E> {
        if let Some(status) = status {
            match status {
                MessageStatus::Sending => {
                    // Avoid looking back past 7 days since messages are very unlikely to be Pending/Sending beyond
                    // that window
                    let one_week_ago = Utc::now() - Duration::days(7);
                    // 'MessageStatus::Sending' is never set on a messageattempt - instead, we check 'next_attempt'
                    // on the returned row
                    query
                        .filter(
                            messageattempt::Column::Id
                                .gte(MessageAttemptId::start_id(one_week_ago)),
                        )
                        .filter(messageattempt::Column::NextAttempt.is_not_null())
                        .filter(
                            // Ensure that we don't include messages that also had a success or failure
                            // in same time period.
                            messageattempt::Column::MsgId.not_in_subquery(
                                limit_messageattempt_join(
                                    <messageattempt::Entity as EntityTrait>::find()
                                        .select_only()
                                        .column(messageattempt::Column::MsgId)
                                        .filter(messageattempt::Column::EndpId.eq(endp_id))
                                        .filter(messageattempt::Column::NextAttempt.is_null())
                                        .filter(messageattempt::Column::Status.is_in(vec![
                                            MessageStatus::Success,
                                            MessageStatus::Fail,
                                        ])),
                                    Some(one_week_ago),
                                    after,
                                    now,
                                )
                                .into_query(),
                            ),
                        )
                }
                // An message with any successful attempts is considered successful - regardless of whether
                // or not there were failed attempts before or after.
                MessageStatus::Success => query.filter(messageattempt::Column::Status.eq(status)),
                // A message with a 'final' failed attempt (and zero successful attempts) is considered failed.
                // This failed attempt could either be the last scheduled attempt, or a manual attempt.
                MessageStatus::Fail => {
                    query
                        .filter(messageattempt::Column::Status.eq(status))
                        .filter(messageattempt::Column::NextAttempt.is_null())
                        .filter(
                            // If there were any successful attempts for this messages, then treat the message as success (even if we also have failed attempts)
                            messageattempt::Column::MsgId.not_in_subquery(
                                limit_messageattempt_join(
                                    <messageattempt::Entity as EntityTrait>::find()
                                        .select_only()
                                        .column(messageattempt::Column::MsgId)
                                        .filter(messageattempt::Column::EndpId.eq(endp_id))
                                        .filter(
                                            messageattempt::Column::Status
                                                .eq(MessageStatus::Success),
                                        ),
                                    before,
                                    after,
                                    now,
                                )
                                .into_query(),
                            ),
                        )
                }
                MessageStatus::Pending => {
                    unreachable!("MessageStatus::Pending should have already been handled")
                }
            }
        } else {
            query
        }
    }

    if let Some(status) = status {
        // We no longer use this status, so we'll never look up anything with it
        if status == MessageStatus::Pending {
            return Ok(Json(ListResponse::empty()));
        }
    }

    msgs_and_dests =
        apply_status_filter(msgs_and_dests, endp.id.clone(), status, before, after, now);

    if let Some(EventTypeNameSet(event_types)) = event_types.clone() {
        msgs_and_dests = msgs_and_dests.filter(message::Column::EventType.is_in(event_types));
    }

    let (msgs_and_attempts, iter_direction) = filter_and_paginate_time_limited(
        msgs_and_dests,
        message::Column::Id,
        limit,
        pagination.iterator.clone(),
        before,
        after,
    );

    let msgs_and_dests = limit_messageattempt_join(msgs_and_attempts, before, after, now);
    let msgs_and_dests = limit_message_join(msgs_and_dests, before, after, now);

    // We've found messages that have at least one matching 'messageattempt' row. Now, lookup the *latest*
    // matching messageattempt row for each messages.
    let results = msgs_and_dests.all(db).await?;
    let mut latest_attempts = messageattempt::Entity::secure_find_by_endpoint(endp.id.clone())
        .filter(messageattempt::Column::MsgId.is_in(results.iter().map(|msg| msg.id.clone())))
        .latest_per_msg();

    // Make sure we apply the 'status' filter (if it exists), so that we get the latest attempt matching
    // the query parameters. All of our other filters only apply to 'message'.
    latest_attempts =
        apply_status_filter(latest_attempts, endp.id.clone(), status, before, after, now);
    latest_attempts = limit_messageattempt_join(latest_attempts, before, after, now);

    let mut latest_attempts: HashMap<_, _> = latest_attempts
        .all(db)
        .await?
        .into_iter()
        .map(|attempt| (attempt.msg_id.clone(), attempt))
        .collect();

    let mut out = Vec::with_capacity(results.len());
    let mut msg_content_map: Option<HashMap<_, _>> = if with_content {
        let msg_ids = results.iter().map(|msg| msg.id.clone());
        Some(
            messagecontent::Entity::secure_find_by_id_in(msg_ids.collect())
                .all(db)
                .await?
                .into_iter()
                .map(|m| (m.id, m.payload))
                .collect(),
        )
    } else {
        None
    };

    for msg in results {
        let msg_content = msg_content_map.as_mut().and_then(|map| map.remove(&msg.id));
        let Some(attempt) = latest_attempts.remove(&msg.id) else {
            tracing::warn!(
                msg_id = msg.id.to_string(),
                "Missing attempt for message in list_attempted_messages"
            );
            continue;
        };
        out.push(EndpointMessageOut::from_attempt_and_msg(
            attempt,
            msg,
            msg_content,
            with_content,
        ));
    }

    Ok(Json(EndpointMessageOut::list_response(
        out,
        limit as usize,
        iter_direction,
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

    let (query, iter_direction) = filter_and_paginate_time_limited(
        query,
        messageattempt::Column::Id,
        limit,
        pagination.iterator,
        before,
        after,
    );

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
        iter_direction,
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

    let (query, iter_direction) = filter_and_paginate_time_limited(
        query,
        messageattempt::Column::Id,
        limit,
        pagination.iterator,
        before,
        after,
    );
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
        iter_direction,
    )))
}

// A type combining information from [`messageattempt::Model`]s and [`endpoint::Model`]s to
// output information on attempted destinations
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageEndpointOut {
    #[serde(flatten)]
    endpoint: super::endpoint::EndpointOutCommon,
    pub id: EndpointId,
    status: MessageStatus,
    next_attempt: Option<DateTime<Utc>>,
}

impl ModelOut for MessageEndpointOut {
    fn id_copy(&self) -> String {
        self.id.0.clone()
    }
}

/// `msg_id`: Use a message id or a message `eventId`
#[aide_annotate(op_id = "v1.message-attempt.list-attempted-destinations")]
async fn list_attempted_destinations(
    State(AppState { ref db, .. }): State<AppState>,
    ValidatedQuery(pagination): ValidatedQuery<
        PaginationDescending<ReversibleIterator<EndpointId>>,
    >,
    Path(ApplicationMsgPath { msg_id, .. }): Path<ApplicationMsgPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<ListResponse<MessageEndpointOut>>> {
    let PaginationLimit(limit) = pagination.limit;
    let iterator: Option<ReversibleIterator<EndpointId>> = pagination.iterator;
    let iter_direction = iterator
        .as_ref()
        .map_or(IteratorDirection::Normal, |iter| iter.direction());

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

    let q = messageattempt::Entity::secure_find_by_msg(msg_id.clone());

    // Add filters for upper/lower bounds of MessageAttemptId
    let now = chrono::Utc::now();
    let msg_attempt_lower_limit = MessageAttemptId::start_id(msg_id.timestamp());
    let msg_attempt_upper_limit = MessageAttemptId::start_id(now + FUTURE_QUERY_LIMIT);

    let q = q
        .filter(messageattempt::Column::Id.gt(msg_attempt_lower_limit))
        .filter(messageattempt::Column::Id.lt(msg_attempt_upper_limit))
        .distinct_on([messageattempt::Column::EndpId])
        .limit(limit);

    let q = match iterator {
        Some(ReversibleIterator::Prev(endp_id)) => q
            .filter(messageattempt::Column::EndpId.lt(endp_id))
            .order_by_desc(messageattempt::Column::EndpId),
        Some(ReversibleIterator::Normal(endp_id)) => q
            .filter(messageattempt::Column::EndpId.gt(endp_id))
            .order_by_asc(messageattempt::Column::EndpId),
        None => q.order_by_asc(messageattempt::Column::EndpId),
    };

    // Get the most recent attempt for each endpoint (this interacts
    // with the 'distinct_on' clause)
    let q = q.order_by_desc(messageattempt::Column::Id);

    let msg_attempts = q.all(db).await?;
    let endp_ids: Vec<_> = msg_attempts.iter().map(|m| m.endp_id.clone()).collect();

    let want_deleted = true;
    let endpoints: HashMap<EndpointId, endpoint::Model> =
        endpoint::Entity::secure_find_by_ids(app.id, endp_ids, want_deleted)
            .all(db)
            .await?
            .into_iter()
            .map(|endp| (endp.id.clone(), endp))
            .collect();

    let results: Vec<MessageEndpointOut> = msg_attempts
        .into_iter()
        .filter_map(|msg_attempt: messageattempt::Model| {
            let endp = endpoints.get(&msg_attempt.endp_id)?.clone();
            Some(MessageEndpointOut {
                id: msg_attempt.endp_id,
                status: msg_attempt.status,
                next_attempt: msg_attempt.next_attempt.map(Into::into),
                endpoint: endp.into(),
            })
        })
        .collect();

    Ok(Json(MessageEndpointOut::list_response(
        results,
        limit as usize,
        iter_direction,
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

    let (query, iter_direction) = filter_and_paginate_time_limited(
        query,
        messageattempt::Column::Id,
        limit,
        pagination.iterator,
        before,
        after,
    );
    let out = query.all(db).await?.into_iter().map(Into::into).collect();

    Ok(Json(MessageAttemptOut::list_response(
        out,
        limit as usize,
        iter_direction,
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

    queue_tx
        .send(
            &MessageTask::new_task(
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
