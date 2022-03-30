// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationId, ApplicationIdOrUid, BaseId, EndpointId, EndpointIdOrUid, EventChannel,
            EventTypeNameSet, MessageAttemptId, MessageAttemptTriggerType, MessageEndpointId,
            MessageId, MessageIdOrUid, MessageStatus, StatusCodeClass,
        },
    },
    db::models::{endpoint, message, messagedestination},
    error::{Error, HttpError, Result},
    queue::{MessageTask, TaskQueueProducer},
    v1::{
        endpoints::message::MessageOut,
        utils::{
            EmptyResponse, ListResponse, MessageListFetchOptions, ModelOut, ReversibleIterator,
            ValidatedQuery,
        },
    },
};
use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};

use hyper::StatusCode;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Expr, Order, Query},
    Condition, DatabaseConnection, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

use svix_server_derive::ModelOut;
use validator::Validate;

use crate::db::models::messageattempt;
use crate::v1::utils::Pagination;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ModelOut)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttemptOut {
    pub response: String,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptedMessageOut {
    #[serde(flatten)]
    pub msg: MessageOut,
    pub status: MessageStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<DateTimeWithTimeZone>,
}

impl ModelOut for AttemptedMessageOut {
    fn id_copy(&self) -> String {
        self.msg.id.0.clone()
    }
}

impl AttemptedMessageOut {
    pub fn from_dest_and_msg(
        dest: messagedestination::Model,
        msg: message::Model,
    ) -> AttemptedMessageOut {
        AttemptedMessageOut {
            msg: msg.into(),
            status: dest.status,
            next_attempt: dest.next_attempt,
        }
    }
}

/// Additional parameters (besides pagination) in the query string for the "List Attempted Messages"
/// enpoint.
#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptedMessagesQueryParameters {
    #[validate]
    channel: Option<EventChannel>,
    status: Option<MessageStatus>,
}

/// Fetches a list of [`AttemptedMessageOut`]s associated with a given app and endpoint.
async fn list_attempted_messages(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<Pagination<MessageId>>,
    ValidatedQuery(ListAttemptedMessagesQueryParameters { channel, status }): ValidatedQuery<
        ListAttemptedMessagesQueryParameters,
    >,
    Path((_app_id, endp_id)): Path<(ApplicationId, EndpointId)>,
    AuthenticatedApplication {
        permissions: _,
        app: _,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<AttemptedMessageOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.take();

    let mut dests_and_msgs = messagedestination::Entity::secure_find_by_endpoint(endp_id)
        .find_also_related(message::Entity)
        .order_by_desc(messagedestination::Column::MsgId)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        dests_and_msgs = dests_and_msgs.filter(messagedestination::Column::MsgId.lt(iterator));
    }

    if let Some(channel) = channel {
        dests_and_msgs =
            dests_and_msgs.filter(Expr::cust_with_values("channels ?? ?", vec![channel]));
    }

    if let Some(status) = status {
        dests_and_msgs = dests_and_msgs.filter(messagedestination::Column::Status.eq(status));
    }

    Ok(Json(AttemptedMessageOut::list_response(
        dests_and_msgs
            .all(db)
            .await?
            .into_iter()
            .map(
                |(dest, msg): (messagedestination::Model, Option<message::Model>)| {
                    let msg = msg.ok_or_else(|| {
                        Error::Database("No associated message with messagedestination".to_owned())
                    })?;
                    Ok(AttemptedMessageOut::from_dest_and_msg(dest, msg))
                },
            )
            .collect::<Result<_>>()?,
        limit as usize,
    )))
}

/// Additional parameters (besides pagination) in the query string for the "List Attempts by
/// Endpoint" enpoint.
#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptsByEndpointQueryParameters {
    status: Option<MessageStatus>,
    status_code_class: Option<StatusCodeClass>,
    #[validate]
    event_types: Option<EventTypeNameSet>,
    #[validate]
    channel: Option<EventChannel>,
}

// Applies filters common to [`list_attempts_by_endpoint`] and [`list_attempts_by_msg`]
fn list_attempts_by_endpoint_or_message_filters(
    mut query: Select<messageattempt::Entity>,
    limit: u64,
    iterator: Option<ReversibleIterator<MessageAttemptId>>,
    status: Option<MessageStatus>,
    status_code_class: Option<StatusCodeClass>,
    event_types: Option<EventTypeNameSet>,
    channel: Option<EventChannel>,
) -> Select<messageattempt::Entity> {
    query = match iterator {
        Some(ReversibleIterator::Prev(id)) => query.filter(
            Condition::any().add(
                messageattempt::Column::Id.in_subquery(
                    Query::select()
                        .column(messageattempt::Column::Id)
                        .and_where(messageattempt::Column::Id.gt(id))
                        .order_by_columns(vec![(messageattempt::Column::Id, Order::Asc)])
                        .limit(limit + 1)
                        .to_owned(),
                ),
            ),
        ),
        Some(ReversibleIterator::Normal(id)) => query.filter(messageattempt::Column::Id.lt(id)),
        None => query,
    };

    query = query
        .limit(limit + 1)
        .order_by_desc(messageattempt::Column::Id);

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
            messageattempt::Entity::belongs_to(message::Entity)
                .from(messageattempt::Column::MsgId)
                .to(message::Column::Id)
                .into(),
        );

        if let Some(EventTypeNameSet(event_types)) = event_types {
            query = query.filter(message::Column::EventType.is_in(event_types));
        }

        if let Some(channel) = channel {
            query = query.filter(Expr::cust_with_values("channels ?? ?", vec![channel]));
        }
    }

    query
}

/// Fetches a list of [`MessageAttemptOut`]s for a given endpoint ID
async fn list_attempts_by_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<
        Pagination<ReversibleIterator<MessageAttemptId>>,
    >,
    ValidatedQuery(ListAttemptsByEndpointQueryParameters {
        status,
        status_code_class,
        event_types,
        channel,
    }): ValidatedQuery<ListAttemptsByEndpointQueryParameters>,
    Path((_app_id, endp_id)): Path<(ApplicationId, EndpointId)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.take();

    // Confirm endpoint ID belongs to the given application
    if endpoint::Entity::secure_find_by_id(app.id, endp_id.clone())
        .one(db)
        .await?
        .is_none()
    {
        return Err(Error::Http(HttpError::not_found(None, None)));
    }

    let query = list_attempts_by_endpoint_or_message_filters(
        messageattempt::Entity::secure_find_by_endpoint(endp_id),
        limit,
        iterator,
        status,
        status_code_class,
        event_types,
        channel,
    );

    Ok(Json(MessageAttemptOut::list_response(
        query.all(db).await?.into_iter().map(Into::into).collect(),
        limit as usize,
    )))
}

/// Flattens in a [`ListAttemptsByEndpointOrMsgQueryParameters`] and adds one extra query parameter
#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptsByMsgQueryParameters {
    status: Option<MessageStatus>,
    status_code_class: Option<StatusCodeClass>,
    #[validate]
    event_types: Option<EventTypeNameSet>,
    #[validate]
    channel: Option<EventChannel>,
    #[validate]
    endpoint_id: Option<EndpointIdOrUid>,
}

/// Fetches a list of [`MessageAttemptOut`]s for a given message ID
async fn list_attempts_by_msg(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<
        Pagination<ReversibleIterator<MessageAttemptId>>,
    >,
    ValidatedQuery(ListAttemptsByMsgQueryParameters {
        status,
        status_code_class,
        event_types,
        channel,
        endpoint_id,
    }): ValidatedQuery<ListAttemptsByMsgQueryParameters>,
    Path((_app_id, msg_id)): Path<(ApplicationId, MessageId)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.take();

    // Confirm message ID belongs to the given application
    if message::Entity::secure_find_by_id(app.id.clone(), msg_id.clone())
        .one(db)
        .await?
        .is_none()
    {
        return Err(Error::Http(HttpError::not_found(None, None)));
    }

    let mut query = list_attempts_by_endpoint_or_message_filters(
        messageattempt::Entity::secure_find_by_msg(msg_id),
        limit,
        iterator,
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
            return Err(Error::Http(HttpError::not_found(None, None)));
        }
    }

    Ok(Json(MessageAttemptOut::list_response(
        query.all(db).await?.into_iter().map(Into::into).collect(),
        limit as usize,
    )))
}

/// A type combining information from [`messagedestination::Model`]s and [`endpoint::Model`]s to
/// output information on attempted destinations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageEndpointOut {
    #[serde(flatten)]
    endpoint: super::endpoint::EndpointOut,
    status: MessageStatus,
    next_attempt: Option<DateTime<Utc>>,
}

impl ModelOut for MessageEndpointOut {
    fn id_copy(&self) -> String {
        self.endpoint.id.0.clone()
    }
}

impl MessageEndpointOut {
    fn from_dest_and_endp(dest: messagedestination::Model, endp: endpoint::Model) -> Self {
        MessageEndpointOut {
            endpoint: endp.into(),
            status: dest.status,
            next_attempt: dest.next_attempt.map(Into::into),
        }
    }
}

async fn list_attempted_destinations(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedQuery(mut pagination): ValidatedQuery<Pagination<MessageEndpointId>>,
    Path((_app_id, msg_id)): Path<(ApplicationId, MessageIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageEndpointOut>>> {
    let limit = pagination.limit;
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
        return Err(Error::Http(HttpError::not_found(None, None)));
    };

    // Fetch the [`messagedestination::Model`] and associated [`endpoint::Model`]
    let mut query = messagedestination::Entity::secure_find_by_msg(msg_id)
        .find_also_related(endpoint::Entity)
        .order_by_desc(messagedestination::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(messagedestination::Column::Id.lt(iterator));
    }

    Ok(Json(AttemptedMessageOut::list_response(
        query
            .all(db)
            .await?
            .into_iter()
            .map(
                |(dest, endp): (messagedestination::Model, Option<endpoint::Model>)| {
                    let endp = endp.ok_or_else(|| {
                        Error::Database("No associated endpoint with messagedestination".to_owned())
                    })?;
                    Ok(MessageEndpointOut::from_dest_and_endp(dest, endp))
                },
            )
            .collect::<Result<_>>()?,
        limit as usize,
    )))
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListAttemptsForEndpointQueryParameters {
    #[validate]
    pub channel: Option<EventChannel>,
    pub status: Option<MessageStatus>,
}

async fn list_attempts_for_endpoint(
    extension: Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<MessageAttemptId>>,
    ValidatedQuery(ListAttemptsForEndpointQueryParameters { channel, status }): ValidatedQuery<
        ListAttemptsForEndpointQueryParameters,
    >,
    list_filter: MessageListFetchOptions,
    Path((app_id, msg_id, endp_id)): Path<(ApplicationIdOrUid, MessageIdOrUid, EndpointIdOrUid)>,
    auth_app: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    list_messageattempts(
        extension,
        pagination,
        ValidatedQuery(AttemptListFetchOptions {
            endpoint_id: Some(endp_id),
            channel,
            status,
        }),
        list_filter,
        Path((app_id, msg_id)),
        auth_app,
    )
    .await
}

#[derive(Debug, Deserialize, Validate)]
pub struct AttemptListFetchOptions {
    #[validate]
    pub endpoint_id: Option<EndpointIdOrUid>,
    #[validate]
    pub channel: Option<EventChannel>,
    pub status: Option<MessageStatus>,
}

// FIXME: add filtering by event_types/channel
async fn list_messageattempts(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<MessageAttemptId>>,
    ValidatedQuery(AttemptListFetchOptions {
        endpoint_id,
        channel,
        status,
    }): ValidatedQuery<AttemptListFetchOptions>,
    list_filter: MessageListFetchOptions,
    Path((_app_id, msg_id)): Path<(ApplicationIdOrUid, MessageIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<MessageAttemptOut>>> {
    let limit = pagination.limit;
    let iterator = pagination
        .iterator
        .clone()
        .or_else(|| list_filter.before.map(MessageAttemptId::start_id));

    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut query = messageattempt::Entity::secure_find_by_msg(msg.id)
        .order_by_desc(messageattempt::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(messageattempt::Column::Id.lt(iterator))
    }

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
        query = query.filter(Expr::cust_with_values("channels ?? ?", vec![channel]));
    }

    if let Some(EventTypeNameSet(event_types)) = list_filter.event_types {
        query = query.filter(message::Column::EventType.is_in(event_types));
    }

    Ok(Json(MessageAttemptOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

async fn get_messageattempt(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, msg_id, attempt_id)): Path<(
        ApplicationIdOrUid,
        MessageIdOrUid,
        MessageAttemptId,
    )>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
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

async fn resend_webhook(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Path((_app_id, msg_id, endp_id)): Path<(ApplicationIdOrUid, MessageIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let msg = message::Entity::secure_find_by_id_or_uid(app.id.clone(), msg_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
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
    Ok((StatusCode::ACCEPTED, Json(EmptyResponse {})))
}

pub fn router() -> Router {
    Router::new().nest(
        "/app/:app_id/",
        Router::new()
            .nest(
                "msg/:msg_id",
                Router::new()
                    // NOTE: [`list_messageattempts`] is deprecated
                    .route("/attempt/", get(list_messageattempts))
                    .route("/attempt/:attempt_id/", get(get_messageattempt))
                    .route("/endpoint/", get(list_attempted_destinations))
                    .route("/endpoint/:endp_id/resend/", post(resend_webhook))
                    // NOTE: [`list_attempts_for_endpoint`] is deprecated
                    .route(
                        "/endpoint/:endp_id/attempt/",
                        get(list_attempts_for_endpoint),
                    ),
            )
            .route("endpoint/:endp_id/msg/", get(list_attempted_messages))
            .route("attempt/endpoint/:endp_id/", get(list_attempts_by_endpoint))
            .route("attempt/msg/:msg_id/", get(list_attempts_by_msg)),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        AttemptListFetchOptions, ListAttemptedMessagesQueryParameters,
        ListAttemptsByEndpointQueryParameters, ListAttemptsByMsgQueryParameters,
        ListAttemptsForEndpointQueryParameters,
    };
    use serde_json::json;
    use validator::Validate;

    const INVALID_CHANNEL: &str = "$$invalid-channel";
    const VALID_CHANNEL: &str = "valid-channel";
    const INVALID_EVENT_TYPES: &[&str] = &["valid-event-type", "&&invalid-event-type"];
    const VALID_EVENT_TYPES: &[&str] = &["valid-event-type", "another-valid-event-type"];
    const INVALID_ENDPOINT_ID: &str = "$$invalid-endpoint";
    const VALID_ENDPOINT_ID: &str = "ep_valid-endpoint";

    #[test]
    fn test_list_attempted_messages_query_params_validation() {
        let q: ListAttemptedMessagesQueryParameters =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptedMessagesQueryParameters =
            serde_json::from_value(json!({ "channel": VALID_CHANNEL })).unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_list_attempts_by_endpoint_query_parameters_validation() {
        let q: ListAttemptsByEndpointQueryParameters =
            serde_json::from_value(json!({ "event_types": INVALID_EVENT_TYPES })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByEndpointQueryParameters =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());
    }

    #[test]
    fn test_list_attempts_by_msg_query_parameters_validation() {
        let q: ListAttemptsByMsgQueryParameters =
            serde_json::from_value(json!({ "event_types": INVALID_EVENT_TYPES })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByMsgQueryParameters =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByMsgQueryParameters =
            serde_json::from_value(json!({ "endpoint_id": INVALID_ENDPOINT_ID })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsByMsgQueryParameters = serde_json::from_value(json!(
            {
                "event_types": VALID_EVENT_TYPES,
                "channel": VALID_CHANNEL,
                "endpoint_id": VALID_ENDPOINT_ID
            }
        ))
        .unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_list_attempts_for_endpoint_query_parameters_validation() {
        let q: ListAttemptsForEndpointQueryParameters =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: ListAttemptsForEndpointQueryParameters =
            serde_json::from_value(json!({ "channel": VALID_CHANNEL })).unwrap();
        q.validate().unwrap();
    }

    #[test]
    fn test_attempt_list_fetch_options_validation() {
        let q: AttemptListFetchOptions =
            serde_json::from_value(json!({ "endpoint_id": INVALID_ENDPOINT_ID })).unwrap();
        assert!(q.validate().is_err());

        let q: AttemptListFetchOptions =
            serde_json::from_value(json!({ "channel": INVALID_CHANNEL })).unwrap();
        assert!(q.validate().is_err());

        let q: AttemptListFetchOptions = serde_json::from_value(json!(
            {
                "endpoint_id": VALID_ENDPOINT_ID,
                "channel": VALID_CHANNEL
            }
        ))
        .unwrap();
        q.validate().unwrap();
    }
}
