// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
mod crud;
mod headers;
mod recovery;
mod secrets;

use crate::{
    cfg::DefaultSignatureType,
    core::{
        cryptography::Encryption,
        permissions,
        types::{
            metadata::Metadata, ApplicationIdOrUid, BaseId, EndpointId, EndpointIdOrUid,
            EndpointSecretInternal, EndpointUid, EventChannelSet, EventTypeNameSet,
            MessageEndpointId, MessageStatus,
        },
    },
    ctx,
    db::models::messagedestination,
    error::{self, HttpError},
    v1::utils::{
        api_not_implemented,
        patch::{
            patch_field_non_nullable, patch_field_nullable, UnrequiredField,
            UnrequiredNullableField,
        },
        validate_no_control_characters, validate_no_control_characters_unrequired,
        validation_error, ModelIn,
    },
};

use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, FromQueryResult, QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, collections::HashSet};
use url::Url;

use svix_server_derive::{ModelIn, ModelOut};
use validator::{Validate, ValidationError};

use crate::core::types::{EndpointHeaders, EndpointHeadersPatch, EndpointSecret};
use crate::db::models::endpoint;

use self::secrets::generate_secret;

pub fn validate_event_types_ids(
    event_types_ids: &EventTypeNameSet,
) -> std::result::Result<(), ValidationError> {
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
) -> std::result::Result<(), ValidationError> {
    match event_types_ids {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(event_type_ids) => validate_event_types_ids(event_type_ids),
    }
}

pub fn validate_channels_endpoint(
    channels: &EventChannelSet,
) -> std::result::Result<(), ValidationError> {
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
) -> std::result::Result<(), ValidationError> {
    match channels {
        UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
        UnrequiredNullableField::Some(channels) => validate_channels_endpoint(channels),
    }
}

pub fn validate_url(val: &str) -> std::result::Result<(), ValidationError> {
    match Url::parse(val) {
        Ok(url) => {
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

        Err(_) => Err(validation_error(
            Some("url"),
            Some("Endpoint URLs must be valid"),
        )),
    }
}

fn validate_url_unrequired(
    val: &UnrequiredField<String>,
) -> std::result::Result<(), ValidationError> {
    match val {
        UnrequiredField::Absent => Ok(()),
        UnrequiredField::Some(val) => validate_url(val),
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate, ModelIn)]
#[serde(rename_all = "camelCase")]
pub struct EndpointIn {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    #[validate(custom = "validate_no_control_characters")]
    pub description: String,

    #[validate(range(min = 1, message = "Endpoint rate limits must be at least one if set"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<EndpointUid>,
    #[validate(custom = "validate_url")]
    pub url: String,
    #[validate(range(min = 1, message = "Endpoint versions must be at least one"))]
    pub version: u16,
    #[serde(default)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub disabled: bool,
    #[serde(rename = "filterTypes")]
    #[validate(custom = "validate_event_types_ids")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types_ids: Option<EventTypeNameSet>,
    #[validate(custom = "validate_channels_endpoint")]
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
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
        model.url = Set(url);
        model.version = Set(version.into());
        model.disabled = Set(disabled);
        model.event_types_ids = Set(event_types_ids);
        model.channels = Set(channels);
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Validate, ModelIn)]
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
    pub url: UnrequiredField<String>,

    #[validate(custom = "validate_minimum_version_patch")]
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
) -> std::result::Result<(), ValidationError> {
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

fn validate_minimum_version_patch(
    version: &UnrequiredField<u16>,
) -> std::result::Result<(), ValidationError> {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointOutCommon {
    pub description: String,
    pub rate_limit: Option<u16>,
    /// Optional unique identifier for the endpoint
    pub uid: Option<EndpointUid>,
    pub url: String,
    pub version: u16,
    pub disabled: bool,
    #[serde(rename = "filterTypes")]
    pub event_types_ids: Option<EventTypeNameSet>,
    pub channels: Option<EventChannelSet>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<endpoint::Model> for EndpointOutCommon {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ModelOut)]
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

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSecretRotateIn {
    #[validate]
    #[serde(default)]
    key: Option<EndpointSecret>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointSecretOut {
    pub key: EndpointSecret,
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoverIn {
    pub since: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersIn {
    #[validate]
    pub headers: EndpointHeaders,
}

impl ModelIn for EndpointHeadersIn {
    type ActiveModel = endpoint::ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel) {
        let EndpointHeadersIn { headers } = self;
        model.headers = Set(Some(headers));
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersOut {
    pub headers: HashMap<String, String>,
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

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointHeadersPatchIn {
    #[validate]
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

#[derive(Deserialize, Serialize)]
pub struct EndpointStatsOut {
    success: i64,
    pending: i64,
    sending: i64,
    fail: i64,
}

#[derive(Debug, FromQueryResult)]
pub struct EndpointStatsQueryOut {
    status: MessageStatus,
    count: i64,
}

async fn endpoint_stats(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    permissions::Application { app }: permissions::Application,
) -> crate::error::Result<Json<EndpointStatsOut>> {
    let endpoint = ctx!(
        crate::db::models::endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
            .one(db)
            .await
    )?
    .ok_or_else(|| HttpError::not_found(None, None))?
    .id;

    let query_out: Vec<EndpointStatsQueryOut> = ctx!(
        messagedestination::Entity::secure_find_by_endpoint(endpoint)
            .select_only()
            .column(messagedestination::Column::Status)
            .column_as(messagedestination::Column::Status.count(), "count")
            .group_by(messagedestination::Column::Status)
            .filter(
                messagedestination::Column::Id.gte(MessageEndpointId::start_id(
                    chrono::Utc::now() - chrono::Duration::days(28),
                )),
            )
            .into_model::<EndpointStatsQueryOut>()
            .all(db)
            .await
    )?;
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

pub fn router() -> Router {
    Router::new()
        .route(
            "/app/:app_id/endpoint/",
            post(crud::create_endpoint).get(crud::list_endpoints),
        )
        .route(
            "/app/:app_id/endpoint/:endp_id/",
            get(crud::get_endpoint)
                .put(crud::update_endpoint)
                .patch(crud::patch_endpoint)
                .delete(crud::delete_endpoint),
        )
        .route(
            "/app/:app_id/endpoint/:endp_id/secret/",
            get(secrets::get_endpoint_secret),
        )
        .route(
            "/app/:app_id/endpoint/:endp_id/secret/rotate/",
            post(secrets::rotate_endpoint_secret),
        )
        .route("/app/:app_id/endpoint/:endp_id/stats/", get(endpoint_stats))
        .route(
            "/app/:app_id/endpoint/:endp_id/send-example/",
            post(api_not_implemented),
        )
        .route(
            "/app/:app_id/endpoint/:endp_id/recover/",
            post(recovery::recover_failed_webhooks),
        )
        .route(
            "/app/:app_id/endpoint/:endp_id/headers/",
            get(headers::get_endpoint_headers)
                .patch(headers::patch_endpoint_headers)
                .put(headers::update_endpoint_headers),
        )
}

#[cfg(test)]
mod tests {

    use crate::core::types::EndpointHeaders;

    use super::{
        validate_url, EndpointHeadersIn, EndpointHeadersOut, EndpointHeadersPatchIn, EndpointIn,
    };
    use serde_json::json;
    use std::collections::{HashMap, HashSet};
    use validator::Validate;

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

    #[test]
    fn test_endpoint_in_validation() {
        let invalid_1: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_INVALID,
             "url": URL_VALID
        }))
        .unwrap();

        let invalid_2: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_INVALID
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

        let invalid_6: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "channels": EVENT_CHANNELS_INVALID
        }))
        .unwrap();

        for e in [
            invalid_1, invalid_2, invalid_3, invalid_4, invalid_5, invalid_6,
        ] {
            assert!(e.validate().is_err());
        }

        let valid: EndpointIn = serde_json::from_value(json!({
             "version": VERSION_VALID,
             "url": URL_VALID,
             "rateLimit": RATE_LIMIT_VALID,
             "uid": ENDPOINT_ID_VALID,
             "filterTypes": EVENT_TYPES_VALID,
             "channels": EVENT_CHANNELS_VALID
        }))
        .unwrap();
        valid.validate().unwrap();
    }

    #[test]
    fn test_endpoint_headers_in_validation() {
        let headers_valid = HashMap::from([("x-valid", "1")]);
        let headers_invalid = HashMap::from([("x-invalid???", "1")]);

        let invalid: EndpointHeadersIn =
            serde_json::from_value(json!({ "headers": headers_invalid })).unwrap();
        assert!(invalid.validate().is_err());

        let valid: EndpointHeadersIn =
            serde_json::from_value(json!({ "headers": headers_valid })).unwrap();
        valid.validate().unwrap();
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
        let valid_https = "https://test.url";
        let valid_http = "http://test.url";
        let invalid_scheme = "anythingelse://test.url";
        let invalid_format = "http://[:::1]";

        assert!(validate_url(valid_https).is_ok());
        assert!(validate_url(valid_http).is_ok());
        assert!(validate_url(invalid_scheme).is_err());
        assert!(validate_url(invalid_format).is_err());

        let valid_https: EndpointIn =
            serde_json::from_value(json!({"url": valid_https, "version": 1})).unwrap();
        let valid_http: EndpointIn =
            serde_json::from_value(json!({"url": valid_http, "version": 1})).unwrap();
        let invalid_scheme: EndpointIn =
            serde_json::from_value(json!({"url": invalid_scheme, "version": 1})).unwrap();
        let invalid_format: EndpointIn =
            serde_json::from_value(json!({"url": invalid_format, "version": 1})).unwrap();

        assert!(valid_https.validate().is_ok());
        assert!(valid_http.validate().is_ok());
        assert!(invalid_scheme.validate().is_err());
        assert!(invalid_format.validate().is_err());
    }
}
