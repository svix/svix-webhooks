// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
mod crud;
mod headers;
mod recovery;
mod secrets;

use std::collections::{HashMap, HashSet};

use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_orm::{ActiveValue::Set, ColumnTrait, FromQueryResult, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use svix_server_derive::{aide_annotate, ModelIn, ModelOut};
use url::Url;
use validator::{Validate, ValidationError};

use self::secrets::generate_secret;
use super::message::{create_message_inner, MessageIn, MessageOut, RawPayload};
use crate::{
    cfg::DefaultSignatureType,
    core::{
        cryptography::Encryption,
        permissions,
        types::{
            metadata::Metadata, BaseId, EndpointHeaders, EndpointHeadersPatch, EndpointId,
            EndpointSecret, EndpointSecretInternal, EndpointUid, EventChannelSet, EventTypeName,
            EventTypeNameSet, MessageEndpointId, MessageStatus,
        },
    },
    db::models::{endpoint, eventtype, messagedestination},
    error::{self, HttpError},
    v1::utils::{
        openapi_tag,
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired,
        validation_error, ApplicationEndpointPath, ModelIn, ValidatedJson,
    },
    AppState,
};

pub fn validate_event_types_ids(event_types_ids: &EventTypeNameSet) -> Result<(), ValidationError> {
    if event_types_ids.0.is_empty() {
        Err(validation_error(
            Some("filterTypes"),
            Some("filterTypes can't be empty, it must have at least one item."),
        ))
    } else {
        Ok(())
    }
}

fn validate_event_types_ids_unrequired_nullable(
    event_types_ids: &UnrequiredNullableField<EventTypeNameSet>,
) -> Result<(), ValidationError> {
    match event_types_ids {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(event_type_ids) => validate_event_types_ids(event_type_ids),
    }
}

pub fn validate_channels_endpoint(channels: &EventChannelSet) -> Result<(), ValidationError> {
    let len = channels.0.len();
    if !(1..=10).contains(&len) {
        Err(validation_error(
            Some("channels"),
            Some("Channels must have at least 1 and at most 10 items, or be set to null."),
        ))
    } else {
        Ok(())
    }
}

fn validate_channels_endpoint_unrequired_nullable(
    channels: &UnrequiredNullableField<EventChannelSet>,
) -> Result<(), ValidationError> {
    match channels {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(channels) => validate_channels_endpoint(channels),
    }
}

pub fn validate_url(url: &Url) -> Result<(), ValidationError> {
    let scheme = url.scheme();
    if scheme == "https" || scheme == "http" {
        Ok(())
    } else {
        Err(validation_error(
            Some("url"),
            Some("Endpoint URL schemes must be http or https"),
        ))
    }
}

fn validate_url_unrequired(val: &UnrequiredField<Url>) -> Result<(), ValidationError> {
    match val {
        UnrequiredField::Absent => Ok(()),
        UnrequiredField::Some(val) => validate_url(val),
    }
}

fn example_channel_set() -> Vec<&'static str> {
    vec!["project_123", "group_2"]
}

fn example_endpoint_description() -> &'static str {
    "An example endpoint name"
}

fn example_filter_types() -> Vec<&'static str> {
    vec!["user.signup", "user.deleted"]
}

fn endpoint_disabled_default() -> bool {
    false
}

fn example_endpoint_url() -> &'static str {
    "https://example.com/webhook/"
}

fn example_endpoint_version() -> u16 {
    1
}

fn default_endpoint_version() -> Option<u16> {
    Some(1)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointIn {
    #[serde(default)]
    #[validate(custom = "validate_no_control_characters")]
    #[schemars(example = "example_endpoint_description")]
    pub description: String,

    #[validate(range(min = 1, message = "Endpoint rate limits must be at least one if set"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<EndpointUid>,

    #[validate(custom = "validate_url")]
    #[schemars(url, length(min = 1, max = 65_536), example = "example_endpoint_url")]
    pub url: Url,

    #[deprecated]
    #[serde(default = "default_endpoint_version")]
    #[validate(range(min = 1, message = "Endpoint versions must be at least one if set"))]
    #[schemars(range(min = 1), example = "example_endpoint_version")]
    pub version: Option<u16>,

    #[serde(default)]
    #[schemars(example = "endpoint_disabled_default")]
    pub disabled: bool,
    #[serde(rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(example = "example_filter_types", length(min = 1))]
    pub event_types_ids: Option<EventTypeNameSet>,
    /// List of message channels this endpoint listens to (omit for all)
    #[validate(custom = "validate_channels_endpoint")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(example = "example_channel_set", length(min = 1, max = 10))]
    pub channels: Option<EventChannelSet>,

    #[validate]
    #[serde(default)]
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<EndpointSecret>,

    #[serde(default)]
    pub metadata: Metadata,
}

impl EndpointIn {
    pub fn key_take_or_generate(
        &mut self,
        encryption: &Encryption,
        sig_type: &DefaultSignatureType,
    ) -> error::Result<EndpointSecretInternal> {
        if let Some(key) = self.key.take() {
            EndpointSecretInternal::from_endpoint_secret(key, encryption)
        } else {
            generate_secret(encryption, sig_type)
        }
    }
}

// FIXME: This can and should be a derive macro
impl ModelIn for EndpointIn {
    type ActiveModel = endpoint::ActiveModel;

    #[allow(deprecated)]
    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointIn {
            description,
            rate_limit,
            uid,
            url,
            version,
            disabled,
            event_types_ids,
            channels,
            key: _,
            metadata: _,
        } = self;

        model.description = Set(description);
        model.rate_limit = Set(rate_limit.map(|x| x.into()));
        model.uid = Set(uid);
        model.url = Set(url.into());
        model.version = Set(version.unwrap_or(1).into());
        model.disabled = Set(disabled);
        model.event_types_ids = Set(event_types_ids);
        model.channels = Set(channels);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct EndpointUpdate {
    #[serde(default)]
    #[validate(custom = "validate_no_control_characters")]
    #[schemars(example = "example_endpoint_description")]
    pub description: String,

    #[validate(range(min = 1, message = "Endpoint rate limits must be at least one if set"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// Optional unique identifier for the endpoint
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<EndpointUid>,

    #[validate(custom = "validate_url")]
    #[schemars(url, length(min = 1, max = 65_536), example = "example_endpoint_url")]
    pub url: Url,

    #[deprecated]
    #[serde(default = "default_endpoint_version")]
    #[validate(range(min = 1, message = "Endpoint versions must be at least one if set"))]
    #[schemars(range(min = 1), example = "example_endpoint_version")]
    pub version: Option<u16>,

    #[serde(default)]
    #[schemars(example = "endpoint_disabled_default")]
    pub disabled: bool,

    #[serde(rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids")]
    #[validate]
    #[schemars(example = "example_filter_types", length(min = 1))]
    pub event_types_ids: Option<EventTypeNameSet>,

    /// List of message channels this endpoint listens to (omit for all)
    #[validate(custom = "validate_channels_endpoint")]
    #[validate]
    #[schemars(example = "example_channel_set", length(min = 1, max = 10))]
    pub channels: Option<EventChannelSet>,

    #[serde(default)]
    pub metadata: Metadata,
}

impl ModelIn for EndpointUpdate {
    type ActiveModel = endpoint::ActiveModel;

    #[allow(deprecated)]
    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointUpdate {
            description,
            rate_limit,
            uid,
            url,
            version,
            disabled,
            event_types_ids,
            channels,
            metadata: _,
        } = self;

        model.description = Set(description);
        model.rate_limit = Set(rate_limit.map(|x| x.into()));
        model.uid = Set(uid);
        model.url = Set(url.into());
        model.version = Set(version.unwrap_or(1).into());
        model.disabled = Set(disabled);
        model.event_types_ids = Set(event_types_ids);
        model.channels = Set(channels);
    }
}

impl EndpointUpdate {
    #[allow(deprecated)]
    pub fn into_in_with_default_key(self) -> EndpointIn {
        let EndpointUpdate {
            description,
            rate_limit,
            uid,
            url,
            version,
            disabled,
            event_types_ids,
            channels,
            metadata,
        } = self;

        EndpointIn {
            description,
            rate_limit,
            uid,
            url,
            version,
            disabled,
            event_types_ids,
            channels,
            metadata,

            key: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Validate, ModelIn, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPatch {
    #[serde(default)]
    #[serde(skip_serializing_if = "UnrequiredField::is_absent")]
    #[validate(custom = "validate_no_control_characters_unrequired")]
    pub description: UnrequiredField<String>,

    #[validate(custom = "validate_rate_limit_patch")]
    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    pub rate_limit: UnrequiredNullableField<u16>,

    #[validate]
    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    pub uid: UnrequiredNullableField<EndpointUid>,

    #[validate(custom = "validate_url_unrequired")]
    #[serde(default)]
    pub url: UnrequiredField<Url>,

    #[deprecated]
    #[validate(custom = "validate_minimum_version_patch")]
    #[schemars(range(min = 1), example = "example_endpoint_version")]
    #[serde(default)]
    pub version: UnrequiredField<u16>,

    #[serde(default)]
    #[serde(skip_serializing_if = "UnrequiredField::is_absent")]
    pub disabled: UnrequiredField<bool>,

    #[serde(default, rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids_unrequired_nullable")]
    #[validate]
    #[serde(skip_serializing_if = "UnrequiredNullableField::is_absent")]
    pub event_types_ids: UnrequiredNullableField<EventTypeNameSet>,

    #[validate(custom = "validate_channels_endpoint_unrequired_nullable")]
    #[validate]
    #[serde(default, skip_serializing_if = "UnrequiredNullableField::is_absent")]
    pub channels: UnrequiredNullableField<EventChannelSet>,

    #[validate]
    #[serde(default)]
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "UnrequiredNullableField::is_absent")]
    pub key: UnrequiredNullableField<EndpointSecret>,

    #[serde(default)]
    #[serde(skip_serializing_if = "UnrequiredField::is_absent")]
    pub metadata: UnrequiredField<Metadata>,
}

impl ModelIn for EndpointPatch {
    type ActiveModel = endpoint::ActiveModel;

    #[allow(deprecated)]
    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointPatch {
            description,
            rate_limit,
            uid,
            url,
            version,
            disabled,
            event_types_ids,
            channels,
            key: _,
            metadata: _,
        } = self;

        let map = |x: u16| -> i32 { x.into() };
        let url = url.map(String::from);

        patch_field_non_nullable!(model, description);
        patch_field_nullable!(model, rate_limit, map);
        patch_field_nullable!(model, uid);
        patch_field_non_nullable!(model, url);
        patch_field_non_nullable!(model, version, map);
        patch_field_non_nullable!(model, disabled);
        patch_field_nullable!(model, event_types_ids);
        patch_field_nullable!(model, channels);
    }
}

fn validate_rate_limit_patch(
    rate_limit: &UnrequiredNullableField<u16>,
) -> Result<(), ValidationError> {
    match rate_limit {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(rate_limit) => {
            if *rate_limit > 0 {
                Ok(())
            } else {
                Err(validation_error(
                    Some("range"),
                    Some("Endpoint rate limits must be at least 1 if set"),
                ))
            }
        }
    }
}

fn validate_minimum_version_patch(version: &UnrequiredField<u16>) -> Result<(), ValidationError> {
    match version {
        UnrequiredField::Absent => Ok(()),
        UnrequiredField::Some(version) => {
            if *version == 0 {
                Err(validation_error(
                    Some("range"),
                    Some("Endpoint versions must be at least one"),
                ))
            } else {
                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointOutCommon {
    /// An example endpoint name
    pub description: String,
    pub rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    pub uid: Option<EndpointUid>,
    #[schemars(url, length(min = 1, max = 65_536), example = "example_endpoint_url")]
    pub url: String,
    #[deprecated]
    #[schemars(range(min = 1), example = "example_endpoint_version")]
    pub version: u16,
    #[schemars(
        example = "endpoint_disabled_default",
        default = "endpoint_disabled_default"
    )]
    pub disabled: bool,
    #[serde(rename = "filterTypes")]
    #[schemars(example = "example_filter_types", length(min = 1))]
    pub event_types_ids: Option<EventTypeNameSet>,
    /// List of message channels this endpoint listens to (omit for all)
    #[schemars(example = "example_channel_set", length(min = 1, max = 10))]
    pub channels: Option<EventChannelSet>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<endpoint::Model> for EndpointOutCommon {
    #[allow(deprecated)]
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
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointOut {
    #[serde(flatten)]
    pub ep: EndpointOutCommon,
    pub id: EndpointId,
    pub metadata: Metadata,
}

// FIXME: This can and should be a derive macro
impl From<(endpoint::Model, Metadata)> for EndpointOut {
    fn from((endp, metadata): (endpoint::Model, Metadata)) -> Self {
        Self {
            id: endp.id.clone(),
            ep: endp.into(),
            metadata,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Validate, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSecretRotateIn {
    #[validate]
    #[serde(default)]
    key: Option<EndpointSecret>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSecretOut {
    pub key: EndpointSecret,
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RecoverIn {
    pub since: DateTime<Utc>,
}

fn endpoint_headers_example() -> HashMap<&'static str, &'static str> {
    HashMap::from([("X-Example", "123"), ("X-Foobar", "Bar")])
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersIn {
    #[schemars(example = "endpoint_headers_example")]
    pub headers: EndpointHeaders,
}

impl ModelIn for EndpointHeadersIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointHeadersIn { headers } = self;
        model.headers = Set(Some(headers));
    }
}

fn sensitive_headers_example() -> HashSet<String> {
    HashSet::from(["Authorization".to_string()])
}

/// The value of the headers is returned in the `headers` field.
///
/// Sensitive headers that have been redacted are returned in the sensitive field.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersOut {
    #[schemars(example = "endpoint_headers_example")]
    pub headers: HashMap<String, String>,
    #[schemars(example = "sensitive_headers_example")]
    pub sensitive: HashSet<String>,
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
        let (sens, remaining) = hdr.0.into_iter().partition(|(k, _)| {
            let k = k.to_lowercase();
            Self::SENSITIVE_HEADERS.iter().any(|&x| x == k)
        });

        Self {
            headers: remaining,
            sensitive: sens.into_keys().collect(),
        }
    }
}

fn endpoint_headers_patch_example() -> EndpointHeadersPatch {
    EndpointHeadersPatch(HashMap::from([
        ("X-Example".to_string(), Some("123".to_string())),
        ("X-Foobar".to_string(), Some("Bar".to_string())),
    ]))
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersPatchIn {
    #[validate]
    #[schemars(example = "endpoint_headers_patch_example")]
    pub headers: EndpointHeadersPatch,
}

impl ModelIn for EndpointHeadersPatchIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointHeadersPatchIn { headers } = self;

        model.headers = if let Some(Some(mut hdrs)) = model.headers.take() {
            for (k, v) in headers.0 {
                if let Some(v) = v {
                    hdrs.0.insert(k, v);
                } else {
                    hdrs.0.remove(&k);
                }
            }
            Set(Some(hdrs))
        } else {
            let headers: HashMap<String, String> = headers
                .0
                .into_iter()
                .filter_map(|(k, v)| v.map(|v| (k, v)))
                .collect();
            Set(Some(EndpointHeaders(headers)))
        };
    }
}

#[derive(Deserialize, JsonSchema)]
struct EndpointStatsRange {
    since: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}

impl EndpointStatsRange {
    fn validate_unwrap_or_default(self) -> error::Result<(DateTime<Utc>, DateTime<Utc>)> {
        let until = self.until.unwrap_or_else(Utc::now);

        if until > Utc::now() {
            return Err(HttpError::bad_request(
                Some("invalid_range".into()),
                Some("'until' cannot be in the future".into()),
            )
            .into());
        }

        let since = self.since.unwrap_or(until - Duration::days(28));

        // Add five minutes so that people can easily just do `now() - 28 days`
        // without having to worry about clock sync
        if until - since > (Duration::days(28) + Duration::minutes(5)) {
            return Err(HttpError::bad_request(
                Some("invalid_range".into()),
                Some(format!(
                    "'since' cannot be more than 28 days prior to {until}"
                )),
            )
            .into());
        }

        Ok((since, until))
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[schemars(rename = "EndpointStats")]
pub struct EndpointStatsOut {
    pub success: i64,
    pub pending: i64,
    pub sending: i64,
    pub fail: i64,
}

#[derive(Debug, FromQueryResult)]
pub struct EndpointStatsQueryOut {
    status: MessageStatus,
    count: i64,
}

/// Get basic statistics for the endpoint.
#[aide_annotate(op_id = "v1.endpoint.get-stats")]
async fn endpoint_stats(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    Query(range): Query<EndpointStatsRange>,
    permissions::Application { app }: permissions::Application,
) -> error::Result<Json<EndpointStatsOut>> {
    let (since, until) = range.validate_unwrap_or_default()?;

    let endpoint =
        crate::db::models::endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
            .one(db)
            .await?
            .ok_or_else(|| HttpError::not_found(None, None))?
            .id;

    let query_out: Vec<EndpointStatsQueryOut> =
        messagedestination::Entity::secure_find_by_endpoint(endpoint)
            .select_only()
            .column(messagedestination::Column::Status)
            .column_as(messagedestination::Column::Status.count(), "count")
            .group_by(messagedestination::Column::Status)
            .filter(messagedestination::Column::Id.gte(MessageEndpointId::start_id(since)))
            .filter(messagedestination::Column::Id.lte(MessageEndpointId::start_id(until)))
            .into_model::<EndpointStatsQueryOut>()
            .all(db)
            .await?;
    let mut query_out = query_out
        .into_iter()
        .map(|EndpointStatsQueryOut { status, count }| (status, count))
        .collect::<HashMap<_, _>>();

    Ok(Json(EndpointStatsOut {
        success: query_out.remove(&MessageStatus::Success).unwrap_or(0),
        pending: query_out.remove(&MessageStatus::Pending).unwrap_or(0),
        fail: query_out.remove(&MessageStatus::Fail).unwrap_or(0),
        sending: query_out.remove(&MessageStatus::Sending).unwrap_or(0),
    }))
}

#[derive(Deserialize, JsonSchema, Validate)]
#[serde(rename_all = "camelCase")]
struct EventExampleIn {
    event_type: EventTypeName,
}

const SVIX_PING_EVENT_TYPE_NAME: &str = "svix.ping";
const SVIX_PING_EVENT_TYPE_PAYLOAD: &str = r#"{"success": true}"#;

/// Send an example message for an event
#[aide_annotate(
    op_id = "v1.endpoint.send-example",
    op_summary = "Send Event Type Example Message"
)]
async fn send_example(
    state: State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::OrganizationWithApplication { app }: permissions::OrganizationWithApplication,
    ValidatedJson(data): ValidatedJson<EventExampleIn>,
) -> error::Result<Json<MessageOut>> {
    let State(AppState {
        ref db,
        queue_tx,
        cache,
        ..
    }) = state;

    let endpoint = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let example = if data.event_type == EventTypeName(SVIX_PING_EVENT_TYPE_NAME.to_owned()) {
        SVIX_PING_EVENT_TYPE_PAYLOAD.to_string()
    } else {
        let event_type =
            eventtype::Entity::secure_find_by_name(app.org_id.clone(), data.event_type.clone())
                .one(db)
                .await?
                .ok_or_else(|| HttpError::not_found(None, None))?;

        let example = event_type.schemas.and_then(|schema| {
            schema
                .example()
                .and_then(|ex| serde_json::to_string(ex).ok())
        });

        match example {
            Some(example) => example,
            None => {
                return Err(HttpError::bad_request(
                    Some("invalid_scheme".to_owned()),
                    Some("Unable to generate example message from event-type schema".to_owned()),
                )
                .into());
            }
        }
    };

    let msg_in = MessageIn {
        channels: None,
        event_type: data.event_type,
        payload: RawPayload::from_string(example).unwrap(),
        uid: None,
        payload_retention_period: 90,
    };

    let create_message =
        create_message_inner(db, queue_tx, cache, false, Some(endpoint.id), msg_in, app).await?;

    Ok(Json(create_message))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Endpoint");
    ApiRouter::new()
        .api_route_with(
            "/app/:app_id/endpoint",
            post_with(crud::create_endpoint, crud::create_endpoint_operation)
                .get_with(crud::list_endpoints, crud::list_endpoints_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id",
            get_with(crud::get_endpoint, crud::get_endpoint_operation)
                .put_with(crud::update_endpoint, crud::update_endpoint_operation)
                .patch_with(crud::patch_endpoint, crud::patch_endpoint_operation)
                .delete_with(crud::delete_endpoint, crud::delete_endpoint_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/secret",
            get_with(
                secrets::get_endpoint_secret,
                secrets::get_endpoint_secret_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/secret/rotate",
            post_with(
                secrets::rotate_endpoint_secret,
                secrets::rotate_endpoint_secret_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/stats",
            get_with(endpoint_stats, endpoint_stats_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/send-example",
            post_with(send_example, send_example_operation),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/recover",
            post_with(
                recovery::recover_failed_webhooks,
                recovery::recover_failed_webhooks_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/app/:app_id/endpoint/:endpoint_id/headers",
            get_with(
                headers::get_endpoint_headers,
                headers::get_endpoint_headers_operation,
            )
            .patch_with(
                headers::patch_endpoint_headers,
                headers::patch_endpoint_headers_operation,
            )
            .put_with(
                headers::update_endpoint_headers,
                headers::update_endpoint_headers_operation,
            ),
            tag,
        )
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use reqwest::Url;
    use serde_json::json;
    use validator::Validate;

    use super::{validate_url, EndpointHeadersOut, EndpointHeadersPatchIn, EndpointIn};
    use crate::core::types::EndpointHeaders;

    const URL_VALID: &str = "https://www.example.com";
    const URL_INVALID: &str = "invalid url";
    const VERSION_VALID: u16 = 1;
    const VERSION_INVALID: u16 = 0;
    const RATE_LIMIT_VALID: u16 = 1;
    const RATE_LIMIT_INVALID: u16 = 0;
    const EVENT_TYPES_INVALID: &[&str] = &["valid-event-type", "&&invalid-event-type"];
    const EVENT_TYPES_VALID: &[&str] = &["valid-event-type1", "valid-event-type2"];
    const EVENT_CHANNELS_INVALID: &[&str] = &["valid-event-channel", "&&invalid-event-channel"];
    const EVENT_CHANNELS_VALID: &[&str] = &["valid-event-channel1", "valid-event-channel2"];
    const ENDPOINT_ID_INVALID: &str = "$$invalid-endpoint";
    const ENDPOINT_ID_VALID: &str = "valid-endpoint";

    #[allow(deprecated)]
    #[test]
    fn test_endpoint_in_validation() {
        let invalid_1: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_INVALID,
             "url": URL_VALID
        }))
        .unwrap();

        let invalid_2: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "channels": EVENT_CHANNELS_INVALID
        }))
        .unwrap();

        let invalid_3: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "rateLimit": RATE_LIMIT_INVALID
        }))
        .unwrap();

        let invalid_4: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "uid": ENDPOINT_ID_INVALID
        }))
        .unwrap();

        let invalid_5: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "filterTypes": EVENT_TYPES_INVALID
        }))
        .unwrap();

        let invalid_6: Result<EndpointIn, _> = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_INVALID
        }));
        assert!(invalid_6.is_err());

        for e in [invalid_1, invalid_2, invalid_3, invalid_4, invalid_5] {
            assert!(e.validate().is_err());
        }

        let valid_1: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "rateLimit": RATE_LIMIT_VALID,
             "uid": ENDPOINT_ID_VALID,
             "filterTypes": EVENT_TYPES_VALID,
             "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();
        valid_1.validate().unwrap();

        let valid_2: EndpointIn = serde_json::from_value(json!({
             "url": URL_VALID,
             "rateLimit": RATE_LIMIT_VALID,
             "uid": ENDPOINT_ID_VALID,
             "filterTypes": EVENT_TYPES_VALID,
             "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();
        valid_2.validate().unwrap();
        assert_eq!(1, valid_2.version.unwrap());
    }

    #[test]
    fn test_endpoint_headers_sensitive() {
        let headers = EndpointHeaders(HashMap::from([
            ("foo".to_string(), "1".to_string()),
            ("authorization".to_string(), "test".to_string()),
            ("X-Auth-Token".to_string(), "test2".to_string()),
        ]));

        let headers_out: EndpointHeadersOut = headers.into();

        assert_eq!(
            headers_out.headers,
            HashMap::from([("foo".to_string(), "1".to_string())])
        );
        assert_eq!(
            headers_out.sensitive,
            HashSet::from(["authorization".to_string(), "X-Auth-Token".to_string()])
        );
    }

    #[test]
    fn test_endpoint_headers_patch_in_validation() {
        let headers_valid = HashMap::from([("x-valid", "1")]);
        let headers_invalid = HashMap::from([("x-invalid???", "1")]);

        let invalid: EndpointHeadersPatchIn =
            serde_json::from_value(json!({ "headers": headers_invalid })).unwrap();
        assert!(invalid.validate().is_err());

        let valid: EndpointHeadersPatchIn =
            serde_json::from_value(json!({ "headers": headers_valid })).unwrap();
        valid.validate().unwrap();
    }

    #[test]
    fn test_url_validation() {
        let valid_https = Url::parse("https://test.url").unwrap();
        let valid_http = Url::parse("http://test.url").unwrap();
        let invalid_scheme = Url::parse("anythingelse://test.url").unwrap();
        let invalid_format = "http://[:::1]";

        assert!(validate_url(&valid_https).is_ok());
        assert!(validate_url(&valid_http).is_ok());
        assert!(validate_url(&invalid_scheme).is_err());

        let valid_https: EndpointIn =
            serde_json::from_value(json!({"url": valid_https, "version": 1})).unwrap();
        let valid_http: EndpointIn =
            serde_json::from_value(json!({"url": valid_http, "version": 1})).unwrap();
        let invalid_scheme: EndpointIn =
            serde_json::from_value(json!({"url": invalid_scheme, "version": 1})).unwrap();
        let invalid_format: Result<EndpointIn, _> =
            serde_json::from_value(json!({"url": invalid_format, "version": 1}));

        assert!(valid_https.validate().is_ok());
        assert!(valid_http.validate().is_ok());
        assert!(invalid_scheme.validate().is_err());
        assert!(invalid_format.is_err());
    }
}
