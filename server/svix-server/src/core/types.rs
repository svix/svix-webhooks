// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use regex::Regex;
use sea_orm::{
    entity::prelude::*,
    sea_query::{ColumnType, Nullable, ValueType, ValueTypeErr},
    TryFromU64, TryGetError, TryGetable,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use svix_ksuid::*;
use validator::{Validate, ValidationError, ValidationErrors};

const ALL_ERROR: &str = "__all__";

macro_rules! enum_wrapper {
    ($name_id:ty) => {
        impl Serialize for $name_id {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_i16((*self).into())
            }
        }

        impl<'de> Deserialize<'de> for $name_id {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error;
                i16::deserialize(deserializer)?.try_into().map_err(|_| {
                    Error::custom(format!("Failed deserializing {}", stringify!($name_id)))
                })
            }
        }

        impl From<$name_id> for sea_orm::Value {
            fn from(v: $name_id) -> Self {
                Self::SmallInt(Some(v.into()))
            }
        }

        impl TryGetable for $name_id {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                match i16::try_get(res, pre, col) {
                    // We are using null as a placeholder error for invalid values
                    Ok(v) => v.try_into().map_err(|_| TryGetError::Null),
                    Err(e) => Err(e),
                }
            }
        }

        impl Nullable for $name_id {
            fn null() -> Value {
                Value::SmallInt(None)
            }
        }

        impl ValueType for $name_id {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::SmallInt(Some(x)) => x.try_into().map_err(|_| ValueTypeErr),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> ColumnType {
                i16::column_type()
            }
        }
    };
}

macro_rules! json_wrapper {
    ($name_id:ty) => {
        impl From<$name_id> for sea_orm::Value {
            fn from(v: $name_id) -> Self {
                let v = serde_json::to_value(v).unwrap();
                Self::Json(Some(Box::new(v)))
            }
        }

        impl TryGetable for $name_id {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                match Json::try_get(res, pre, col) {
                    Ok(v) => Ok(serde_json::from_value(v).unwrap()),
                    Err(e) => Err(e),
                }
            }
        }

        impl Nullable for $name_id {
            fn null() -> Value {
                Value::Json(None)
            }
        }

        impl ValueType for $name_id {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::Json(Some(x)) => Ok(serde_json::from_value(*x).unwrap()),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> ColumnType {
                ColumnType::JsonBinary
            }
        }
    };
}

pub trait BaseId: Deref<Target = String> {
    const PREFIX: &'static str;
    type Output;

    fn validate_(&self) -> std::result::Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        if !&self.starts_with(Self::PREFIX) {
            errors.add(
                ALL_ERROR,
                ValidationError::new("Invalid id. Expected different prefix"),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn generate_(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> String {
        let ksuid = Ksuid::new(dt, payload);
        format!("{}{}", Self::PREFIX, ksuid.to_string())
    }

    fn new(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self::Output;

    fn start_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0u8; Ksuid::PAYLOAD_BYTES];
        Self::new(Some(start), Some(&buf[..]))
    }

    fn end_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0xFFu8; Ksuid::PAYLOAD_BYTES];
        Self::new(Some(start), Some(&buf[..]))
    }
}

fn validate_limited_str(s: &str) -> std::result::Result<(), ValidationErrors> {
    const MAX_LENGTH: usize = 256;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9\-_.]+$").unwrap();
    }
    let mut errors = ValidationErrors::new();
    if s.len() > MAX_LENGTH {
        errors.add(ALL_ERROR, ValidationError::new("String too long"));
    } else if !RE.is_match(s) {
        errors.add(
            ALL_ERROR,
            ValidationError::new("String must match the following pattern: [a-zA-Z0-9\\-_.]."),
        );
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub trait BaseUid: Deref<Target = String> {
    const ID_PREFIX: &'static str;

    fn validate_(&self) -> std::result::Result<(), ValidationErrors> {
        let mut errors = match validate_limited_str(self) {
            Ok(_) => ValidationErrors::new(),
            Err(x) => x,
        };
        if self.starts_with(Self::ID_PREFIX) {
            errors.add(
                ALL_ERROR,
                ValidationError::new(
                    "Uids are not allowed to have the same prefix as the ID. Prefix with _?",
                ),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

macro_rules! string_wrapper {
    ($name_id:ident) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name_id(pub String);

        impl Deref for $name_id {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name_id> for sea_orm::Value {
            fn from(v: $name_id) -> Self {
                Self::String(Some(Box::new(v.0)))
            }
        }

        impl TryGetable for $name_id {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                match String::try_get(res, pre, col) {
                    Ok(v) => Ok($name_id(v)),
                    Err(e) => Err(e),
                }
            }
        }

        impl Nullable for $name_id {
            fn null() -> Value {
                Value::String(None)
            }
        }

        impl ValueType for $name_id {
            fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
                match v {
                    Value::String(Some(x)) => Ok($name_id(*x)),
                    _ => Err(ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> ColumnType {
                String::column_type()
            }
        }

        impl std::fmt::Display for $name_id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

macro_rules! create_id_type {
    ($name_id:ident, $key_prefix:literal) => {
        string_wrapper!($name_id);

        impl BaseId for $name_id {
            const PREFIX: &'static str = $key_prefix;
            type Output = Self;

            fn new(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self::Output {
                Self(Self::generate_(dt, payload))
            }
        }

        impl Validate for $name_id {
            fn validate(&self) -> Result<(), validator::ValidationErrors> {
                self.validate_()
            }
        }

        impl TryFromU64 for $name_id {
            fn try_from_u64(_: u64) -> Result<Self, DbErr> {
                Err(DbErr::Exec(format!(
                    "{} cannot be converted from u64",
                    stringify!($type)
                )))
            }
        }
    };
}

macro_rules! create_all_id_types {
    ($name_id:ident, $name_uid:ident, $name_id_or_uid:ident, $key_prefix:literal) => {
        // Id
        create_id_type!($name_id, $key_prefix);

        // Uid
        string_wrapper!($name_uid);

        impl BaseUid for $name_uid {
            const ID_PREFIX: &'static str = $key_prefix;
        }

        impl Validate for $name_uid {
            fn validate(&self) -> Result<(), validator::ValidationErrors> {
                self.validate_()
            }
        }

        impl From<$name_uid> for $name_id_or_uid {
            fn from(v: $name_uid) -> Self {
                Self(v.0)
            }
        }

        // Id or uid
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct $name_id_or_uid(pub String);

        impl From<$name_id_or_uid> for $name_uid {
            fn from(v: $name_id_or_uid) -> Self {
                Self(v.0)
            }
        }

        impl From<$name_id_or_uid> for $name_id {
            fn from(v: $name_id_or_uid) -> Self {
                Self(v.0)
            }
        }

        impl From<$name_id_or_uid> for sea_orm::Value {
            fn from(v: $name_id_or_uid) -> Self {
                Self::String(Some(Box::new(v.0)))
            }
        }
    };
}

create_id_type!(OrganizationId, "org_");
create_id_type!(MessageAttemptId, "atmpt_");
create_id_type!(MessageEndpointId, "msgep_");
create_id_type!(EventTypeId, "evtype_");

create_all_id_types!(ApplicationId, ApplicationUid, ApplicationIdOrUid, "app_");
create_all_id_types!(EndpointId, EndpointUid, EndpointIdOrUid, "ep_");
create_all_id_types!(MessageId, MessageUid, MessageIdOrUid, "msg_");

string_wrapper!(EventTypeName);

impl Validate for EventTypeName {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_limited_str(&self.0)
    }
}

string_wrapper!(EventChannel);

impl Validate for EventChannel {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validate_limited_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventChannelSet(pub HashSet<EventChannel>);
json_wrapper!(EventChannelSet);

impl Validate for EventChannelSet {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        for item in self.0.iter() {
            item.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventTypeNameSet(pub HashSet<EventTypeName>);
json_wrapper!(EventTypeNameSet);

impl Validate for EventTypeNameSet {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        for item in self.0.iter() {
            item.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpiringSigningKeys(pub Vec<ExpiringSigningKey>);
json_wrapper!(ExpiringSigningKeys);

impl ExpiringSigningKeys {
    pub const MAX_OLD_KEYS: usize = 5;
    pub const OLD_KEY_EXPIRY_HOURS: i64 = 24;
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndpointSecret(pub Vec<u8>);
impl EndpointSecret {
    const PREFIX: &'static str = "whsec_";
}

impl EndpointSecret {
    const KEY_SIZE: usize = 24;

    pub fn generate() -> crate::error::Result<Self> {
        let mut buf = [0u8; Self::KEY_SIZE];
        getrandom::getrandom(&mut buf)?;
        Ok(Self(buf.to_vec()))
    }
}

impl Serialize for EndpointSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}{}", Self::PREFIX, &base64::encode(&self.0[..]))[..])
    }
}

impl<'de> Deserialize<'de> for EndpointSecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        String::deserialize(deserializer)
            .and_then(|string| {
                base64::decode(&string[Self::PREFIX.len()..])
                    .map_err(|err| Error::custom(err.to_string()))
            })
            .map(EndpointSecret)
    }
}

impl From<EndpointSecret> for sea_orm::Value {
    fn from(v: EndpointSecret) -> Self {
        Self::Bytes(Some(Box::new(v.0)))
    }
}

impl TryGetable for EndpointSecret {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match Vec::<u8>::try_get(res, pre, col) {
            Ok(v) => Ok(EndpointSecret(v)),
            Err(e) => Err(e),
        }
    }
}

impl Nullable for EndpointSecret {
    fn null() -> Value {
        Value::Bytes(None)
    }
}

impl ValueType for EndpointSecret {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Bytes(Some(x)) => Ok(EndpointSecret(*x)),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(EndpointSecret).to_owned()
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(None)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpiringSigningKey {
    #[serde(rename = "signingKey")]
    pub key: EndpointSecret,
    pub expiration: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EndpointHeaders(pub HashMap<String, String>);
json_wrapper!(EndpointHeaders);

impl<'de> Deserialize<'de> for EndpointHeaders {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        HashMap::deserialize(deserializer)
            .map(|x: HashMap<String, String>| {
                x.into_iter().map(|(k, v)| (k.to_lowercase(), v)).collect()
            })
            .map(EndpointHeaders)
    }
}

impl Validate for EndpointHeaders {
    fn validate(&self) -> std::result::Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        self.0.iter().for_each(|(k, v)| {
            if let Err(_e) = http::header::HeaderName::try_from(k) {
                errors.add(ALL_ERROR, ValidationError::new("Invalid Header Name."));
            }
            if let Err(_e) = http::header::HeaderValue::try_from(v) {
                errors.add(ALL_ERROR, ValidationError::new("Invalid Header Value."));
            }
        });
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[repr(i16)]
#[derive(Clone, Debug, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MessageAttemptTriggerType {
    Scheduled = 0,
    Manual = 1,
}

#[repr(i16)]
#[derive(Clone, Debug, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum MessageStatus {
    Success = 0,
    Pending = 1,
    Fail = 2,
    Sending = 3,
}

enum_wrapper!(MessageAttemptTriggerType);
enum_wrapper!(MessageStatus);

#[cfg(test)]
mod tests {
    use crate::core::types::{EventChannel, EventTypeName};

    use super::{ApplicationId, ApplicationUid, EndpointHeaders};
    use std::collections::HashMap;
    use validator::Validate;

    #[test]
    fn test_id_validation() {
        let app_id = ApplicationId("app_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        app_id.validate().unwrap();

        let app_id = ApplicationId("badprefix_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        assert!(app_id.validate().is_err());

        let app_uid = ApplicationUid("app_24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        assert!(app_uid.validate().is_err());

        let app_uid = ApplicationUid("24NVKcPqNLXKu3xQhJnw8fSumZK".to_owned());
        app_uid.validate().unwrap();

        // With a space
        let app_uid = ApplicationUid("24NVKcPqNLXKu3 ".to_owned());
        assert!(app_uid.validate().is_err());

        // Check all allowed
        let app_uid = ApplicationUid("azAZ09-_.".to_owned());
        app_uid.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let app_id = ApplicationUid(long_str.clone());
        assert!(app_id.validate().is_err());
        let app_uid = ApplicationUid(long_str);
        assert!(app_uid.validate().is_err());
    }

    #[test]
    fn test_event_names_validation() {
        // With a space
        let evt_name = EventTypeName("event ".to_owned());
        assert!(evt_name.validate().is_err());

        // Check all allowed
        let evt_name = EventTypeName("azAZ09-_.".to_owned());
        evt_name.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let evt_name = EventTypeName(long_str);
        assert!(evt_name.validate().is_err());
    }

    #[test]
    fn test_event_channel_validation() {
        // With a space
        let evt_name = EventChannel("event ".to_owned());
        assert!(evt_name.validate().is_err());

        // Check all allowed
        let evt_name = EventChannel("azAZ09-_.".to_owned());
        evt_name.validate().unwrap();

        // Check length
        let long_str: String = "X".repeat(300);
        let evt_name = EventChannel(long_str);
        assert!(evt_name.validate().is_err());
    }

    #[test]
    fn test_endpoint_headers_validation() {
        let hdr_map = HashMap::from([
            ("valid".to_owned(), "true".to_owned()),
            ("also-valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        endpoint_headers.validate().unwrap();

        let hdr_map = HashMap::from([
            ("invalid?".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([
            ("invalid\0".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(endpoint_headers.validate().is_err());
    }

    #[test]
    fn test_endpoint_headers_deserialization() {
        let js = r#"
            {
                "NOT_UPPER_CASE": "TRUE",
                "is_lower_case": "true"
            }
        "#;
        let eph: EndpointHeaders = serde_json::from_str(js).unwrap();
        assert_eq!(
            HashMap::from([
                ("not_upper_case".to_owned(), "TRUE".to_owned()),
                ("is_lower_case".to_owned(), "true".to_owned()),
            ]),
            eph.0
        );
    }
}
