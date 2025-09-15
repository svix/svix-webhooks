// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    borrow::Cow,
    collections::HashSet,
    error::Error as StdError,
    ops::Deref,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use aide::{
    transform::{TransformOperation, TransformPathItem},
    OperationInput, OperationIo, OperationOutput,
};
use axum::{
    async_trait,
    extract::{
        rejection::{BytesRejection, FailedToBufferBody},
        FromRequest, FromRequestParts, Query, Request,
    },
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use http::{request::Parts, StatusCode};
use regex::Regex;
use schemars::JsonSchema;
use sea_orm::{ColumnTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    core::types::{
        ApplicationIdOrUid, BaseId, EndpointIdOrUid, EventTypeName, EventTypeNameSet,
        MessageAttemptId, MessageIdOrUid,
    },
    error::{Error, HttpError, Result, ValidationErrorItem},
};

pub mod patch;
use patch::UnrequiredField;

const fn default_limit() -> PaginationLimit {
    PaginationLimit(50)
}

const PAGINATION_LIMIT_CAP_HARD: bool = true;
const PAGINATION_LIMIT_CAP_LIMIT: u64 = 250;
static PAGINATION_LIMIT_ERROR: LazyLock<String> =
    LazyLock::new(|| format!("Given limit must not exceed {PAGINATION_LIMIT_CAP_LIMIT}"));

static FUTURE_QUERY_LIMIT: LazyLock<chrono::Duration> =
    LazyLock::new(|| chrono::Duration::hours(1));
static LIMITED_QUERY_DURATION: LazyLock<chrono::Duration> =
    LazyLock::new(|| chrono::Duration::days(90));

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
pub struct PaginationDescending<T: Validate + JsonSchema> {
    /// Limit the number of returned items
    #[validate]
    #[serde(default = "default_limit")]
    pub limit: PaginationLimit,
    /// The iterator returned from a prior invocation
    #[validate]
    pub iterator: Option<T>,
}

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
pub struct Pagination<T: Validate + JsonSchema> {
    /// Limit the number of returned items
    #[validate]
    #[serde(default = "default_limit")]
    pub limit: PaginationLimit,
    /// The iterator returned from a prior invocation
    #[validate]
    pub iterator: Option<T>,
    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Clone, Debug, JsonSchema)]
#[schemars(transparent)]
pub struct PaginationLimit(pub u64);

impl<'de> Deserialize<'de> for PaginationLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let limit = u64::deserialize(deserializer)?;

        // Want hard limits to stay the same so they can be validated
        if !PAGINATION_LIMIT_CAP_HARD && limit > PAGINATION_LIMIT_CAP_LIMIT {
            Ok(PaginationLimit(PAGINATION_LIMIT_CAP_LIMIT))
        } else {
            Ok(PaginationLimit(limit))
        }
    }
}

impl Validate for PaginationLimit {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errs = validator::ValidationErrors::new();

        if self.0 > PAGINATION_LIMIT_CAP_LIMIT {
            errs.add(
                "limit",
                validation_error(Some("pagination"), Some(&PAGINATION_LIMIT_ERROR)),
            );
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReversibleIterator<T: Validate> {
    /// Regular iteration - backwards in time.
    Normal(T),
    /// Reversed iteration - forwards in time.
    Prev(T),
}

impl<T: Validate> ReversibleIterator<T> {
    pub(crate) fn direction(&self) -> IteratorDirection {
        match self {
            Self::Normal(_) => IteratorDirection::Normal,
            Self::Prev(_) => IteratorDirection::Prev,
        }
    }
}

impl<'de, T: 'static + Deserialize<'de> + Validate + From<String>> Deserialize<'de>
    for ReversibleIterator<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| {
            if let Some(s) = s.strip_prefix('-') {
                ReversibleIterator::Prev(T::from(s.to_owned()))
            } else {
                ReversibleIterator::Normal(T::from(s))
            }
        })
    }
}

impl<T: Validate> Validate for ReversibleIterator<T> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            ReversibleIterator::Normal(val) => val.validate(),
            ReversibleIterator::Prev(val) => val.validate(),
        }
    }
}

impl<T: Validate + JsonSchema> JsonSchema for ReversibleIterator<T> {
    fn schema_name() -> String {
        format!("ReversibleIterator_{}", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        T::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        false
    }
}

/// Applies sorting and filtration to a query from its iterator, sort column, and limit
/// queries based on time
/// Our rules for limiting queries are as follows
///
/// If `before` is passed:
/// * lower limit on query is `before - LIMITED_QUERY_DURATION`
/// * upper limit is `before`
///
/// If `after` is passed:
/// * lower limit is `after`
/// * upper limit is `now + FUTURE_QUERY_LIMIT`
///
/// If prev-iterator is passed:
/// * lower limit is `prev-iterator`
/// * upper limit is `prev-iterator + LIMITED_QUERY_DURATION`
///
/// If (normal) iterator is passed:
/// * lower limit is `iterator - LIMITED_QUERY_DURATION`
/// * upper limit is `iterator`
///
/// If no iterator is passed:
/// * lower limit is `now() - LIMITED_QUERY_DURATION` if
///   neither `before` nor `after` were passed
pub(crate) fn filter_and_paginate_time_limited<Q, I>(
    mut query: Q,
    sort_column: impl ColumnTrait,
    limit: u64,
    iterator: Option<ReversibleIterator<I>>,
    before: Option<DateTime<Utc>>,
    after: Option<DateTime<Utc>>,
) -> (Q, IteratorDirection)
where
    Q: QuerySelect + QueryOrder + QueryFilter,
    I: BaseId<Output = I> + Validate + Into<sea_orm::Value>,
{
    let mut limit_time = true;
    if let Some(before) = before {
        if limit_time {
            query = query.filter(sort_column.gt(I::start_id(before - *LIMITED_QUERY_DURATION)));
            limit_time = false;
        }
        query = query.filter(sort_column.lt(I::start_id(before)));
    }

    if let Some(after) = after {
        if limit_time {
            query = query.filter(sort_column.lt(I::end_id(after + *LIMITED_QUERY_DURATION)));
            limit_time = false;
        }
        query = query.filter(sort_column.gt(I::start_id(after)));
    }

    let (mut query, iter_direction) = match (&iterator, before, after) {
        (Some(ReversibleIterator::Prev(_)), _, _) | (None, None, Some(_)) => {
            (query.order_by_asc(sort_column), IteratorDirection::Prev)
        }
        _ => (query.order_by_desc(sort_column), IteratorDirection::Normal),
    };

    let now = chrono::Utc::now();
    let future_limit = now + *FUTURE_QUERY_LIMIT;
    match iterator {
        Some(ReversibleIterator::Prev(id)) => {
            let ts = id.timestamp();
            query = query.filter(sort_column.gt(id));
            if limit_time {
                query = query.filter(sort_column.lt(I::end_id(ts + *LIMITED_QUERY_DURATION)));
            }
        }

        Some(ReversibleIterator::Normal(id)) => {
            let ts = id.timestamp();
            query = query.filter(sort_column.lt(id));
            if limit_time {
                query = query.filter(sort_column.gt(I::start_id(ts - *LIMITED_QUERY_DURATION)));
            }
        }

        None => {
            if limit_time {
                query = query.filter(sort_column.gt(I::start_id(now - *LIMITED_QUERY_DURATION)));
            }
        }
    }

    query = query
        // Query for an extra element to be able to tell whether there's more
        // data than the user requested.
        .limit(limit + 1)
        // Blanket limit on future
        .filter(sort_column.lt(I::start_id(future_limit)));

    (query, iter_direction)
}

/// Marker trait for any type that is used for iterating through results
/// in the public API.
pub trait IdIterator: Validate + Into<sea_orm::Value> {}

impl<T: BaseId + Validate + Into<sea_orm::Value>> IdIterator for T {}
impl IdIterator for EventTypeName {}

pub fn apply_pagination<
    Q: QuerySelect + QueryOrder + QueryFilter,
    C: ColumnTrait,
    I: IdIterator,
>(
    query: Q,
    sort_column: C,
    limit: u64,
    iterator: Option<ReversibleIterator<I>>,
    ordering: Ordering,
) -> Q {
    use Ordering::*;
    use ReversibleIterator::*;

    // Query for an extra element to be able to tell whether there's more
    // data than the user requested.
    let query = query.limit(limit + 1);

    let iterator = if let Some(it) = iterator {
        it
    } else {
        return match ordering {
            Ascending => query.order_by_asc(sort_column),
            Descending => query.order_by_desc(sort_column),
        };
    };

    match (iterator, ordering) {
        (Prev(id), Ascending) | (Normal(id), Descending) => {
            query.order_by_desc(sort_column).filter(sort_column.lt(id))
        }
        (Prev(id), Descending) | (Normal(id), Ascending) => {
            query.order_by_asc(sort_column).filter(sort_column.gt(id))
        }
    }
}

/// A response with no body content and a specific response code, specified by
/// the generic parameter `N`.
pub struct NoContentWithCode<const N: u16>;

impl<const N: u16> IntoResponse for NoContentWithCode<N> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::from_u16(N).unwrap(), ()).into_response()
    }
}

impl<const N: u16> OperationOutput for NoContentWithCode<N> {
    type Inner = Self;

    fn operation_response(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        <() as OperationOutput>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        if let Some(response) = Self::operation_response(ctx, operation) {
            vec![(Some(N), response)]
        } else {
            vec![]
        }
    }
}

/// A response with no body content and HTTP status code 204, the standard code
/// for such responses.
#[derive(OperationIo)]
#[aide(output_with = "()")]
pub struct NoContent;

impl IntoResponse for NoContent {
    fn into_response(self) -> axum::response::Response {
        NoContentWithCode::<204>::into_response(NoContentWithCode)
    }
}

#[derive(Serialize, JsonSchema)]
pub struct EmptyResponse {}

// If you change the internal representation of this then you must also update
// it in the `JsonSchema` impl below to match.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub iterator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
    pub done: bool,
}

impl<T> ListResponse<T> {
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            iterator: None,
            prev_iterator: None,
            done: true,
        }
    }
}

// This custom impl is needed because we want to customize the name of the
// schema that goes into the spec, but that can only be done by having a custom
// `JsonSchema` implementation.
// Tracking issue: https://github.com/GREsau/schemars/issues/193
impl<T: JsonSchema> JsonSchema for ListResponse<T> {
    fn schema_name() -> String {
        let data_type_name = T::schema_name();
        format!("ListResponse_{data_type_name}_")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        fn example_iterator() -> &'static str {
            "iterator"
        }

        fn example_prev_iterator() -> &'static str {
            "-iterator"
        }

        // The actual schema generation is still delegated to the derive macro.
        #[derive(JsonSchema)]
        #[allow(unused)]
        #[serde(rename_all = "camelCase")]
        struct ListResponse<T> {
            pub data: Vec<T>,
            #[schemars(example = "example_iterator")]
            pub iterator: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schemars(example = "example_prev_iterator")]
            pub prev_iterator: Option<String>,
            pub done: bool,
        }

        ListResponse::<T>::json_schema(gen)
    }
}

pub trait ModelIn {
    type ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel);
}

/// Defines the ordering in a listing of results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Ordering {
    Ascending,
    Descending,
}

#[derive(PartialEq, Eq)]
pub(crate) enum IteratorDirection {
    /// Regular iteration - backwards in time.
    Normal,
    /// Reversed iteration - forwards in time.
    Prev,
}

fn list_response_inner<T: ModelOut>(
    mut data: Vec<T>,
    limit: usize,
    iter_direction: IteratorDirection,
    supports_prev_iterator: bool,
) -> ListResponse<T> {
    // Our queries use a LIMIT of (limit + 1), so if there is more data than
    // the user requested, `data.len()` is going to be larger than limit.
    let done = data.len() <= limit;

    // Drop the excess element(s). Should be only one.
    data.truncate(limit);

    // If iterating forwards in time, the query requests the DB rows in reverse
    // to what the user actually wants.
    if iter_direction == IteratorDirection::Prev {
        data.reverse();
    }

    let prev_iterator = if supports_prev_iterator {
        data.first().map(|x| format!("-{}", x.id_copy()))
    } else {
        None
    };
    let iterator = data.last().map(|x| x.id_copy());

    ListResponse {
        data,
        iterator,
        prev_iterator,
        done,
    }
}

pub(crate) trait ModelOut: Sized {
    fn id_copy(&self) -> String;

    fn list_response(
        data: Vec<Self>,
        limit: usize,
        direction: IteratorDirection,
    ) -> ListResponse<Self> {
        list_response_inner(data, limit, direction, true)
    }
}

// Helper method to simplify the somewhat egregious API for creating a ValidationError
pub fn validation_error(code: Option<&'static str>, msg: Option<&'static str>) -> ValidationError {
    ValidationError {
        code: std::borrow::Cow::from(code.unwrap_or("validation")),
        message: msg.map(std::borrow::Cow::from),
        params: std::collections::HashMap::new(),
    }
}

/// Recursively searches a [`validator::ValidationErrors`] tree into a linear list of errors to be
/// sent to the user
fn validation_errors(
    acc_path: Vec<String>,
    err: validator::ValidationErrors,
) -> Vec<ValidationErrorItem> {
    err.into_errors()
        .into_iter()
        .flat_map(|(k, v)| {
            // Add the next field to the location
            let mut loc = acc_path.clone();
            loc.push(k.to_owned());

            match v {
                // If it's a [`validator::ValidationErrorsKind::Field`], then it will be a vector of
                // errors to map to [`ValidationErrorItem`]s and insert to [`out`] before the next
                // iteration
                validator::ValidationErrorsKind::Field(vec) => vec
                    .into_iter()
                    .map(|err| ValidationErrorItem {
                        loc: loc.clone(),
                        msg: err
                            .message
                            .unwrap_or(Cow::Borrowed("Validation error"))
                            .to_string(),
                        ty: "value_error".to_owned(),
                    })
                    .collect(),
                // If it is a [`validator::ValidationErrorsKind::Struct`], then it will be another
                // [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::Struct(errors) => validation_errors(loc, *errors),

                // If it is a [`validator::ValidationErrorsKind::List`], then it will be an
                // [`std::collections::BTreeMap`] of [`validator::ValidationErrors`] to search
                validator::ValidationErrorsKind::List(map) => map
                    .into_iter()
                    .flat_map(|(k, v)| {
                        // Add the list index to the location
                        let mut loc = loc.clone();
                        loc.push(format!("[{k}]"));

                        validation_errors(loc, *v)
                    })
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default, OperationIo)]
#[aide(input_with = "axum::extract::Json<T>", json_schema)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self> {
        let b = bytes::Bytes::from_request(req, state).await.map_err(|e| {
            tracing::error!("Error reading body as bytes: {}", e);

            match e {
                BytesRejection::FailedToBufferBody(FailedToBufferBody::LengthLimitError(_)) => {
                    HttpError::too_large(None, None)
                }

                _ => HttpError::internal_server_error(
                    None,
                    Some("Failed to read request body".to_owned()),
                ),
            }
        })?;
        let mut de = serde_json::Deserializer::from_slice(&b);

        let value: T = serde_path_to_error::deserialize(&mut de).map_err(|e| {
            let mut path = e
                .path()
                .to_string()
                .split('.')
                .map(ToOwned::to_owned)
                .collect::<Vec<String>>();
            let inner = e.inner();

            let mut loc = vec!["body".to_owned()];
            loc.append(&mut path);
            HttpError::unprocessable_entity(vec![ValidationErrorItem {
                loc,
                msg: inner
                    .source()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| e.to_string()),
                ty: "value_error.jsondecode".to_owned(),
            }])
        })?;

        value.validate().map_err(|e| {
            HttpError::unprocessable_entity(validation_errors(vec!["body".to_owned()], e))
        })?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|err| HttpError::bad_request(None, Some(err.to_string())))?;
        value.validate().map_err(|e| {
            HttpError::unprocessable_entity(validation_errors(vec!["query".to_owned()], e))
        })?;
        Ok(ValidatedQuery(value))
    }
}

impl<T> Deref for ValidatedQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: JsonSchema> OperationInput for ValidatedQuery<T> {
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
        axum::extract::Query::<T>::operation_input(ctx, operation)
    }
}

// A special wrapper to handle query parameter lists. serde_qs and serde_urlencode can't
// handle url query param arrays as flexibly as we need to support in our API
pub struct EventTypesQueryParams(pub Option<EventTypeNameSet>);

#[async_trait]
impl<S> FromRequestParts<S> for EventTypesQueryParams
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let pairs = form_urlencoded::parse(parts.uri.query().unwrap_or_default().as_bytes());

        let event_types: HashSet<EventTypeName> = pairs
            .filter(|(key, _)|
                // want to handle both `?event_types=`, `?event_types[]=`, and `?event_types[1]=`
                key == "event_types" || (key.starts_with("event_types[") && key.ends_with(']')))
            .flat_map(|(_, value)| {
                value
                    .split(',')
                    .map(|x| EventTypeName(x.to_owned()))
                    .collect::<Vec<_>>()
            })
            .collect();

        if event_types.is_empty() {
            Ok(Self(None))
        } else {
            let event_types = EventTypeNameSet(event_types);
            event_types.validate().map_err(|e| {
                HttpError::unprocessable_entity(validation_errors(vec!["query".to_owned()], e))
            })?;
            Ok(Self(Some(event_types)))
        }
    }
}

impl OperationInput for EventTypesQueryParams {
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
        // This struct must match what `EventTypesQuery` would be if we used a
        // simple `#[derive(Deserialize)]` on it.
        #[derive(JsonSchema)]
        struct EventTypesQueryParams {
            /// Filter response based on the event type
            #[allow(unused)]
            event_types: Option<EventTypeNameSet>,
        }

        Query::<EventTypesQueryParams>::operation_input(ctx, operation);
    }
}

pub async fn api_not_implemented() -> Result<()> {
    Err(HttpError::not_implemented(None, None).into())
}

pub fn validate_no_control_characters(str: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"[\x00-\x08]").unwrap();
    if re.is_match(str) {
        return Err(validation_error(
            Some("illegal_character"),
            Some("Control characters 0x00-0x08 not allowed."),
        ));
    }
    Ok(())
}

pub fn validate_no_control_characters_unrequired(
    str: &UnrequiredField<String>,
) -> Result<(), ValidationError> {
    match str {
        UnrequiredField::Absent => Ok(()),
        UnrequiredField::Some(str) => validate_no_control_characters(str),
    }
}

pub fn openapi_tag<T: AsRef<str>>(
    tag: T,
) -> impl Fn(TransformPathItem<'_>) -> TransformPathItem<'_> {
    move |op| op.tag(tag.as_ref())
}

pub fn openapi_desc<T: AsRef<str>>(
    desc: T,
) -> impl Fn(TransformOperation<'_>) -> TransformOperation<'_> {
    move |op| op.description(desc.as_ref())
}

pub fn get_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationPath {
    pub app_id: ApplicationIdOrUid,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationEndpointPath {
    pub app_id: ApplicationIdOrUid,
    pub endpoint_id: EndpointIdOrUid,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationMsgPath {
    pub app_id: ApplicationIdOrUid,
    pub msg_id: MessageIdOrUid,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationMsgEndpointPath {
    pub app_id: ApplicationIdOrUid,
    pub msg_id: MessageIdOrUid,
    pub endpoint_id: EndpointIdOrUid,
}

#[derive(Deserialize, JsonSchema)]
pub struct ApplicationMsgAttemptPath {
    pub app_id: ApplicationIdOrUid,
    pub msg_id: MessageIdOrUid,
    pub attempt_id: MessageAttemptId,
}

#[derive(Deserialize, JsonSchema)]
pub struct EventTypeNamePath {
    pub event_type_name: EventTypeName,
}

/// JsonStatus is a wrapper over `axum::extract::Json` as a handler output.
///
/// Setting the `STATUS` const parameter automatically sets the response
/// status code, as well as inserting it into the aide documentation.
pub struct JsonStatus<const STATUS: u16, T: JsonSchema + Serialize>(pub T);

impl<const STATUS: u16, T: JsonSchema + Serialize> IntoResponse for JsonStatus<STATUS, T> {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(STATUS).unwrap(),
            axum::extract::Json(self.0),
        )
            .into_response()
    }
}

impl<const STATUS: u16, T: JsonSchema + Serialize> OperationOutput for JsonStatus<STATUS, T> {
    type Inner = T;

    fn operation_response(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        axum::extract::Json::<T>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        if let Some(resp) = Self::operation_response(ctx, operation) {
            vec![(Some(STATUS), resp)]
        } else {
            vec![]
        }
    }
}

/// JsonStatusUpsert is a wrapper over `axum::extract::Json` as a handler
/// output.
///
/// It is a special casing of `JsonStatus` for situations where a resource is
/// either being updated or created within the same operation. In case of
/// `Updated` HTTP 200 OK is returned, in case of `Created` HTTP 201 CREATED
/// is returned.
pub enum JsonStatusUpsert<T: JsonSchema + Serialize> {
    Updated(T),
    Created(T),
}

impl<T: JsonSchema + Serialize> IntoResponse for JsonStatusUpsert<T> {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            JsonStatusUpsert::Updated(v) => (StatusCode::OK, v),
            JsonStatusUpsert::Created(v) => (StatusCode::CREATED, v),
        };
        (status, axum::extract::Json(body)).into_response()
    }
}

impl<T: JsonSchema + Serialize> OperationOutput for JsonStatusUpsert<T> {
    type Inner = T;

    fn operation_response(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        axum::extract::Json::<T>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        if let Some(resp) = Self::operation_response(ctx, operation) {
            vec![
                (Some(StatusCode::OK.into()), resp.clone()),
                (Some(StatusCode::CREATED.into()), resp),
            ]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use validator::Validate;

    use super::{default_limit, validate_no_control_characters, validation_errors, Pagination};
    use crate::{core::types::ApplicationUid, error::ValidationErrorItem};

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStruct {
        #[validate(range(min = 10, message = "Below 10"))]
        a: u32,

        #[validate]
        b: ValidationErrorTestStructInner,

        #[validate]
        c: Vec<ValidationErrorTestStructInner>,
    }

    #[derive(Debug, Validate)]
    struct ValidationErrorTestStructInner {
        #[validate(range(max = 10, message = "Above 10"))]
        inner: u8,
    }

    #[test]
    fn test_validation_errors_fn() {
        let valid = ValidationErrorTestStruct {
            a: 11,
            b: ValidationErrorTestStructInner { inner: 1 },
            c: vec![
                ValidationErrorTestStructInner { inner: 2 },
                ValidationErrorTestStructInner { inner: 3 },
            ],
        };
        let invalid = ValidationErrorTestStruct {
            a: 9,
            b: ValidationErrorTestStructInner { inner: 11 },
            c: vec![
                ValidationErrorTestStructInner { inner: 12 },
                ValidationErrorTestStructInner { inner: 13 },
            ],
        };

        assert_eq!(valid.validate(), Ok(()));

        let errs = invalid.validate().unwrap_err();
        let errs = validation_errors(vec![], errs);

        assert_eq!(errs.len(), 4);

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["a".to_owned()],
            msg: "Below 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["b".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));

        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[0]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
        assert!(errs.contains(&ValidationErrorItem {
            loc: vec!["c".to_owned(), "[1]".to_owned(), "inner".to_owned()],
            msg: "Above 10".to_owned(),
            ty: "value_error".to_owned(),
        }));
    }

    #[test]
    fn test_pagination_defaults() {
        let p: Pagination<ApplicationUid> = serde_json::from_value(json!({})).unwrap();
        assert_eq!(p.limit.0, default_limit().0);
    }

    #[test]
    fn test_pagination_validation() {
        let p: Pagination<ApplicationUid> =
            serde_json::from_value(json!({"iterator": "$$invalid-appuid"})).unwrap();
        assert!(p.validate().is_err());

        let p: Pagination<ApplicationUid> =
            serde_json::from_value(json!({ "iterator": "valid-appuid"})).unwrap();
        p.validate().unwrap();
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestPaginationDeserializationStruct {
        iterator: super::ReversibleIterator<crate::core::types::MessageId>,
    }

    #[test]
    fn test_pagination_deserialization() {
        let a = serde_json::json!({"iterator": "msg_274DTsX0wVTSLvo91QopQgZrjDV"});
        let b = serde_json::json!({"iterator": "-msg_274DTsX0wVTSLvo91QopQgZrjDV"});

        assert_eq!(
            serde_json::from_value::<TestPaginationDeserializationStruct>(a).unwrap(),
            TestPaginationDeserializationStruct {
                iterator: super::ReversibleIterator::Normal(crate::core::types::MessageId(
                    "msg_274DTsX0wVTSLvo91QopQgZrjDV".to_owned()
                ))
            }
        );
        assert_eq!(
            serde_json::from_value::<TestPaginationDeserializationStruct>(b).unwrap(),
            TestPaginationDeserializationStruct {
                iterator: super::ReversibleIterator::Prev(crate::core::types::MessageId(
                    "msg_274DTsX0wVTSLvo91QopQgZrjDV".to_owned()
                ))
            }
        );
    }

    #[test]
    fn test_validate_no_control_characters() {
        let a = "A good string";
        let b = "A\u{0000} bad string";

        assert!(validate_no_control_characters(a).is_ok());
        assert!(validate_no_control_characters(b).is_err());
    }
}
