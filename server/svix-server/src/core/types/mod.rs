// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::Rng;

use regex::Regex;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use svix_ksuid::*;
use validator::{Validate, ValidationErrors};

use crate::{err_generic, v1::utils::validation_error};

pub mod metadata;

use super::cryptography::{AsymmetricKey, Encryption};

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

        impl From<$name_id> for sea_orm::entity::prelude::Value {
            fn from(v: $name_id) -> Self {
                Self::SmallInt(Some(v.into()))
            }
        }

        impl sea_orm::TryGetable for $name_id {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                match i16::try_get(res, pre, col) {
                    // We are using null as a placeholder error for invalid sea_orm::Values
                    Ok(v) => v
                        .try_into()
                        .map_err(|_| sea_orm::TryGetError::Null("invalid sea_orm_value".into())),
                    Err(e) => Err(e),
                }
            }
        }

        impl sea_orm::sea_query::Nullable for $name_id {
            fn null() -> sea_orm::Value {
                sea_orm::Value::SmallInt(None)
            }
        }

        impl sea_orm::sea_query::ValueType for $name_id {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::SmallInt(Some(x)) => {
                        x.try_into().map_err(|_| sea_orm::sea_query::ValueTypeErr)
                    }
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                i16::column_type()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                i16::array_type()
            }
        }
    };
}

#[macro_export]
macro_rules! json_wrapper {
    ($name_id:ty) => {
        impl From<$name_id> for sea_orm::entity::prelude::Value {
            fn from(v: $name_id) -> Self {
                let v = serde_json::to_value(v).expect("Error serializing JSON");
                Self::Json(Some(Box::new(v)))
            }
        }

        impl sea_orm::TryGetable for $name_id {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                match sea_orm::prelude::Json::try_get(res, pre, col) {
                    Ok(v) => Ok(serde_json::from_value(v).expect("Error deserializing JSON")),
                    Err(e) => Err(e),
                }
            }
        }

        impl sea_orm::sea_query::Nullable for $name_id {
            fn null() -> sea_orm::Value {
                sea_orm::Value::Json(None)
            }
        }

        impl sea_orm::sea_query::ValueType for $name_id {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::Json(Some(x)) => {
                        Ok(serde_json::from_value(*x).expect("Error deserializing JSON"))
                    }
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                sea_orm::sea_query::ColumnType::JsonBinary
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                sea_orm::sea_query::ArrayType::Json
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
                validation_error(Some("id"), Some("Invalid id. Expected different prefix")),
            );
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn generate_(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> String {
        let ksuid = KsuidMs::new(dt, payload);
        format!("{}{}", Self::PREFIX, ksuid.to_string())
    }

    fn new(dt: Option<DateTime<Utc>>, payload: Option<&[u8]>) -> Self::Output;

    fn start_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0u8; KsuidMs::PAYLOAD_BYTES];
        Self::new(Some(start), Some(&buf[..]))
    }

    fn end_id(start: DateTime<Utc>) -> Self::Output {
        let buf = [0xFFu8; KsuidMs::PAYLOAD_BYTES];
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
        errors.add(
            ALL_ERROR,
            validation_error(Some("length"), Some("String too long")),
        );
    } else if !RE.is_match(s) {
        errors.add(
            ALL_ERROR,
            validation_error(
                Some("illegal_string_pattern"),
                Some("String must match the following pattern: [a-zA-Z0-9\\-_.]."),
            ),
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
                validation_error(
                    Some("invalid_uid_prefix"),
                    Some("Uids are not allowed to have the same prefix as the ID. Prefix with _?"),
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
        #[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
        pub struct $name_id(pub String);

        string_wrapper_impl!($name_id);
    };
}

macro_rules! string_wrapper_impl {
    ($name_id:ident) => {
        impl $name_id {
            /// Wraps the type as JSONB. Useful when doing comparisons in a jsonb container w/sea_orm|postgres.
            pub fn jsonb(self) -> sea_orm::Value {
                sea_orm::Value::Json(Some(Box::new(serde_json::Value::String(self.0))))
            }
        }

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

        impl sea_orm::TryGetable for $name_id {
            fn try_get(
                res: &sea_orm::QueryResult,
                pre: &str,
                col: &str,
            ) -> Result<Self, sea_orm::TryGetError> {
                match String::try_get(res, pre, col) {
                    Ok(v) => Ok($name_id(v)),
                    Err(e) => Err(e),
                }
            }
        }

        impl sea_orm::sea_query::Nullable for $name_id {
            fn null() -> sea_orm::Value {
                sea_orm::Value::String(None)
            }
        }

        impl sea_orm::sea_query::ValueType for $name_id {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::String(Some(x)) => Ok($name_id(*x)),
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!($name_id).to_owned()
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                String::column_type()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                String::array_type()
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

        impl sea_orm::TryFromU64 for $name_id {
            fn try_from_u64(_: u64) -> Result<Self, sea_orm::DbErr> {
                Err(sea_orm::DbErr::Exec(sea_orm::error::RuntimeErr::Internal(
                    format!("{} cannot be converted from u64", stringify!($type)),
                )))
            }
        }

        impl From<String> for $name_id {
            fn from(s: String) -> Self {
                $name_id(s)
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
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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

        impl Validate for $name_id_or_uid {
            fn validate(&self) -> Result<(), validator::ValidationErrors> {
                validate_limited_str(&self.0)
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiringSigningKeys(pub Vec<ExpiringSigningKey>);
json_wrapper!(ExpiringSigningKeys);

impl ExpiringSigningKeys {
    pub const MAX_OLD_KEYS: usize = 5;
    pub const OLD_KEY_EXPIRY_HOURS: i64 = 24;
}

/// The type of encryption key
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum EndpointSecretType {
    Hmac256 = 1,
    Ed25519 = 2,
    // Reserved = 3,
}

impl EndpointSecretType {
    pub const fn secret_prefix(&self) -> &'static str {
        match self {
            EndpointSecretType::Hmac256 => "whsec_",
            EndpointSecretType::Ed25519 => "whsk_",
        }
    }

    pub const fn public_prefix(&self) -> &'static str {
        match self {
            EndpointSecretType::Hmac256 => "whsec_",
            EndpointSecretType::Ed25519 => "whpk_",
        }
    }
}

/// Properties of the encryption key
#[derive(Clone, Debug, PartialEq, Eq)]
struct EndpointSecretMarker {
    type_: EndpointSecretType,
    encrypted: bool,
}

impl EndpointSecretMarker {
    const ENCRYPTED_FLAG: u8 = 0b1000_0000;

    fn from_u8(v: u8) -> crate::error::Result<Self> {
        let encrypted = (v & Self::ENCRYPTED_FLAG) != 0;
        let v = v & !Self::ENCRYPTED_FLAG;
        let type_ =
            EndpointSecretType::try_from(v).map_err(|_| err_generic!("Invalid marker value"))?;

        Ok(Self { type_, encrypted })
    }

    fn to_u8(&self) -> u8 {
        let mut ret = self.type_.clone().into();
        if self.encrypted {
            ret |= Self::ENCRYPTED_FLAG;
        }
        ret
    }

    fn type_(&self) -> &EndpointSecretType {
        &self.type_
    }
}

/// The internal representation of the endpoint secret.
/// This is used to store it securely in the database and cache, and to ensure it doesn't get
/// sent externally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointSecretInternal {
    marker: EndpointSecretMarker,

    key: Vec<u8>,
}

impl EndpointSecretInternal {
    // IMPORTANT: has to be at least 24 bytes because of how we encode the type (and legacy ones
    // didn't have type encoded).
    // XXX Also: can't change withuot breaking from_vec
    const KEY_SIZE: usize = 24;
    // Needed because of rust limitations
    const KEY_SIZE_MINUS_ONE: usize = Self::KEY_SIZE - 1;

    fn new(
        encryption: &Encryption,
        type_: EndpointSecretType,
        key: &[u8],
    ) -> crate::error::Result<Self> {
        Ok(Self {
            marker: EndpointSecretMarker {
                type_,
                encrypted: encryption.enabled(),
            },
            key: encryption.encrypt(key)?,
        })
    }

    pub fn generate_symmetric(encryption: &Encryption) -> crate::error::Result<Self> {
        let buf: [u8; Self::KEY_SIZE] = rand::thread_rng().gen();
        Self::new(encryption, EndpointSecretType::Hmac256, &buf)
    }

    pub fn generate_asymmetric(encryption: &Encryption) -> crate::error::Result<Self> {
        let key = AsymmetricKey::generate();
        Self::new(encryption, EndpointSecretType::Ed25519, key.0.sk.as_slice())
    }

    fn into_vec(mut self) -> Vec<u8> {
        let marker: u8 = self.marker.to_u8();

        let mut vec = vec![marker];
        vec.append(&mut self.key);
        vec
    }

    fn from_vec(v: Vec<u8>) -> crate::error::Result<Self> {
        // Legacy had exact size
        match v.len() {
            0..=Self::KEY_SIZE_MINUS_ONE => Err(err_generic!("Value too small")),
            Self::KEY_SIZE => Ok(Self {
                marker: EndpointSecretMarker {
                    type_: EndpointSecretType::Hmac256,
                    encrypted: false,
                },
                key: v,
            }),
            _ => {
                let marker = EndpointSecretMarker::from_u8(v[0])?;
                Ok(Self {
                    marker,
                    key: v[1..].to_vec(),
                })
            }
        }
    }

    pub fn into_endpoint_secret(
        self,
        encryption: &Encryption,
    ) -> crate::error::Result<EndpointSecret> {
        let key = self.key(encryption)?;
        Ok(match self.type_() {
            EndpointSecretType::Hmac256 => EndpointSecret::Symmetric(key),
            EndpointSecretType::Ed25519 => {
                EndpointSecret::Asymmetric(AsymmetricKey::from_slice(&key[..])?)
            }
        })
    }

    pub fn from_endpoint_secret(
        endpoint_secret: EndpointSecret,
        encryption: &Encryption,
    ) -> crate::error::Result<Self> {
        Ok(match endpoint_secret {
            EndpointSecret::Symmetric(key) => {
                Self::new(encryption, EndpointSecretType::Hmac256, &key)?
            }
            EndpointSecret::Asymmetric(key) => {
                Self::new(encryption, EndpointSecretType::Ed25519, key.0.sk.as_slice())?
            }
        })
    }

    pub fn sign(&self, encryption: &Encryption, bytes: &[u8]) -> Vec<u8> {
        let key = self.key(encryption).unwrap();
        // FIXME: remove unwrap
        match self.marker.type_() {
            EndpointSecretType::Hmac256 => hmac_sha256::HMAC::mac(bytes, key).to_vec(),
            EndpointSecretType::Ed25519 => AsymmetricKey::from_slice(&key[..])
                .unwrap()
                .0
                .sk
                .sign(bytes, None)
                .to_vec(),
        }
    }

    fn key(&self, encryption: &Encryption) -> crate::error::Result<Vec<u8>> {
        Ok(if self.marker.encrypted {
            if encryption.enabled() {
                encryption.decrypt(&self.key)?
            } else {
                return Err(err_generic!("main_secret unset, can't decrypt key"));
            }
        } else {
            self.key.to_vec()
        })
    }

    pub fn type_(&self) -> &EndpointSecretType {
        self.marker.type_()
    }
}

impl Serialize for EndpointSecretInternal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(self.clone().into_vec()))
    }
}

impl<'de> Deserialize<'de> for EndpointSecretInternal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        String::deserialize(deserializer).and_then(|string| {
            // For backwards compat when loading from ExpiringSigningKeys. Going forward we just b64 it
            if string.starts_with(EndpointSecretType::Hmac256.secret_prefix()) {
                Ok(Self {
                    marker: EndpointSecretMarker {
                        type_: EndpointSecretType::Hmac256,
                        encrypted: false,
                    },
                    key: string
                        .get(EndpointSecretType::Hmac256.secret_prefix().len()..)
                        .ok_or_else(|| Error::custom("invalid prefix".to_string()))
                        .and_then(|string| {
                            base64::decode(string).map_err(|err| Error::custom(err.to_string()))
                        })?,
                })
            } else {
                let buf = base64::decode(string).map_err(|err| Error::custom(err.to_string()))?;
                Self::from_vec(buf).map_err(|err| Error::custom(err.to_string()))
            }
        })
    }
}

impl From<EndpointSecretInternal> for sea_orm::Value {
    fn from(v: EndpointSecretInternal) -> Self {
        Self::Bytes(Some(Box::new(v.into_vec())))
    }
}

impl sea_orm::TryGetable for EndpointSecretInternal {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        match Vec::<u8>::try_get(res, pre, col) {
            Ok(v) => EndpointSecretInternal::from_vec(v)
                .map_err(|x| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Type(x.to_string()))),
            Err(e) => Err(e),
        }
    }
}

impl sea_orm::sea_query::Nullable for EndpointSecretInternal {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Bytes(None)
    }
}

impl sea_orm::sea_query::ValueType for EndpointSecretInternal {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::Bytes(Some(x)) => {
                EndpointSecretInternal::from_vec(*x).map_err(|_| sea_orm::sea_query::ValueTypeErr)
            }
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(EndpointSecretInternal).to_owned()
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Binary(sea_orm::sea_query::BlobSize::Blob(None))
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
}

/// The external representation of the endpoint secret.
/// This one is used for serializing to and from customers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EndpointSecret {
    Symmetric(Vec<u8>),
    Asymmetric(AsymmetricKey),
}

impl EndpointSecret {
    // IMPORTANT: has to be at least 24 bytes because of how we encode the type (and legacy ones
    // didn't have type encoded).
    // XXX Also: can't change withuot breaking from_vec
    const KEY_SIZE: usize = 24;
    // Needed because of rust limitations
    const KEY_SIZE_MAX: usize = 75;

    pub fn serialize_secret_key(&self) -> String {
        match self {
            Self::Symmetric(key) => {
                format!(
                    "{}{}",
                    EndpointSecretType::Hmac256.secret_prefix(),
                    base64::encode(key)
                )
            }
            Self::Asymmetric(key) => {
                format!(
                    "{}{}",
                    EndpointSecretType::Ed25519.secret_prefix(),
                    &base64::encode(key.0.sk.as_slice())
                )
            }
        }
    }

    pub fn serialize_public_key(&self) -> String {
        match self {
            Self::Symmetric(key) => {
                format!(
                    "{}{}",
                    EndpointSecretType::Hmac256.public_prefix(),
                    base64::encode(key)
                )
            }
            Self::Asymmetric(key) => {
                format!(
                    "{}{}",
                    EndpointSecretType::Ed25519.public_prefix(),
                    &base64::encode(key.pubkey())
                )
            }
        }
    }
}

impl Serialize for EndpointSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.serialize_public_key())
    }
}

impl<'de> Deserialize<'de> for EndpointSecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let invalid_prefix = Error::custom("invalid prefix".to_string());
        String::deserialize(deserializer).and_then(|string| {
            if string.starts_with(EndpointSecretType::Ed25519.secret_prefix()) {
                Ok(Self::Asymmetric(
                    AsymmetricKey::from_base64(
                        string
                            .get(EndpointSecretType::Ed25519.secret_prefix().len()..)
                            .ok_or(invalid_prefix)?,
                    )
                    .map_err(|e| Error::custom(e.to_string()))?,
                ))
            } else if string.starts_with(EndpointSecretType::Hmac256.secret_prefix()) {
                Ok(Self::Symmetric(
                    string
                        .get(EndpointSecretType::Hmac256.secret_prefix().len()..)
                        .ok_or(invalid_prefix)
                        .and_then(|string| {
                            base64::decode(string).map_err(|err| Error::custom(err.to_string()))
                        })?,
                ))
            } else {
                Err(invalid_prefix)
            }
        })
    }
}

impl Validate for EndpointSecret {
    fn validate(&self) -> std::result::Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        match self {
            Self::Symmetric(bytes) => {
                if bytes.len() < Self::KEY_SIZE || bytes.len() > Self::KEY_SIZE_MAX {
                    errors.add(
                        ALL_ERROR,
                        validation_error(Some("length"), Some("secret length invalid")),
                    );
                }
            }
            Self::Asymmetric(key) => {
                let test_msg = b"123";
                let signature = key.0.sk.sign(test_msg, None);
                if key.0.pk.verify(test_msg, &signature).is_err() {
                    errors.add(
                        ALL_ERROR,
                        validation_error(
                            Some("invalid_key"),
                            Some("Invalid key, failed signing test msg"),
                        ),
                    );
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl JsonSchema for EndpointSecret {
    fn schema_name() -> String {
        "EndpointSecret".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        const KEY_PATTERN: &str = "^(whsec_)?[a-zA-Z0-9+/=]{32,100}$";
        let mut schema = String::json_schema(gen);
        if let schemars::schema::Schema::Object(ref mut obj) = schema {
            obj.string = Some(Box::new(schemars::schema::StringValidation {
                pattern: Some(KEY_PATTERN.to_string()),
                ..Default::default()
            }));
        }
        schema
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiringSigningKey {
    #[serde(rename = "signingKey")]
    pub key: EndpointSecretInternal,
    pub expiration: DateTime<Utc>,
}

const FORBIDDEN_KEYS: [&str; 19] = [
    "user-agent",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailers",
    "transfer-encoding",
    "upgrade",
    "age",
    "cache-control",
    "clear-site-data",
    "expires",
    "pragma",
    "warning",
    "content-length",
    "content-type",
    "content-encoding",
    "content-language",
    "content-location",
];

const FORBIDDEN_PREFIXES: [&str; 10] = [
    "x-amz-", "x-amzn-", "x-google", "x-goog-", "x-gfe", "x-amz-", "x-azure-", "x-fd-", "x-svix-",
    "svix-",
];

fn validate_header_key(k: &str, errors: &mut ValidationErrors) {
    let k = &k.to_lowercase();
    if let Err(_e) = http::header::HeaderName::try_from(k) {
        errors.add(
            ALL_ERROR,
            validation_error(Some("header"), Some("Invalid Header Name.")),
        );
    }
    if FORBIDDEN_KEYS.contains(&k.as_str()) {
        errors.add(
            ALL_ERROR,
            validation_error(Some("header"), Some("Header uses a forbidden key.")),
        );
    }
    FORBIDDEN_PREFIXES.iter().for_each(|p| {
        if k.starts_with(p) {
            errors.add(
                ALL_ERROR,
                validation_error(
                    Some("header"),
                    Some("Header starts with a forbidden prefix."),
                ),
            )
        }
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default, JsonSchema)]
pub struct EndpointHeaders(pub HashMap<String, String>);
json_wrapper!(EndpointHeaders);

const HEADER_MAX_LENGTH: usize = 4096;

impl<'de> Deserialize<'de> for EndpointHeaders {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let headers: HashMap<String, String> = HashMap::deserialize(deserializer)?;

        validate_header_map(&headers).map_err(serde::de::Error::custom)?;

        Ok(EndpointHeaders(headers))
    }
}

fn validate_header_map(
    headers: &HashMap<String, String>,
) -> std::result::Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();
    for (k, v) in headers {
        validate_header_key(k, &mut errors);

        if let Err(_e) = http::header::HeaderValue::try_from(v) {
            errors.add(
                ALL_ERROR,
                validation_error(Some("header"), Some("Invalid Header Value.")),
            );
        }

        if v.len() > HEADER_MAX_LENGTH {
            errors.add(
                ALL_ERROR,
                validation_error(Some("header"), Some("Maximum header length is 4096 bytes")),
            );
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default, JsonSchema)]
pub struct EndpointHeadersPatch(pub HashMap<String, Option<String>>);
json_wrapper!(EndpointHeadersPatch);

impl<'de> Deserialize<'de> for EndpointHeadersPatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        HashMap::deserialize(deserializer)
            .map(|x: HashMap<String, Option<String>>| x.into_iter().collect())
            .map(EndpointHeadersPatch)
    }
}

impl Validate for EndpointHeadersPatch {
    fn validate(&self) -> std::result::Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        self.0
            .iter()
            .for_each(|(k, _)| validate_header_key(k, &mut errors));
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[repr(i16)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, JsonSchema)]
pub enum MessageAttemptTriggerType {
    Scheduled = 0,
    Manual = 1,
}

#[repr(i16)]
#[derive(Clone, Debug, Copy, PartialEq, IntoPrimitive, TryFromPrimitive, Hash, Eq, JsonSchema)]
pub enum MessageStatus {
    Success = 0,
    Pending = 1,
    Fail = 2,
    Sending = 3,
}

#[repr(i16)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, JsonSchema)]
pub enum StatusCodeClass {
    CodeNone = 0,
    Code1xx = 100,
    Code2xx = 200,
    Code3xx = 300,
    Code4xx = 400,
    Code5xx = 500,
}

enum_wrapper!(MessageAttemptTriggerType);
enum_wrapper!(MessageStatus);
enum_wrapper!(StatusCodeClass);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, JsonSchema)]
pub struct FeatureFlag(pub String);

string_wrapper_impl!(FeatureFlag);

impl<'de> Deserialize<'de> for FeatureFlag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| {
            validate_limited_str(&s).map_err(serde::de::Error::custom)?;
            Ok(FeatureFlag(s))
        })
    }
}

pub type FeatureFlagSet = HashSet<FeatureFlag>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{EventChannel, EventTypeName};

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
        validate_header_map(&endpoint_headers.0).unwrap();

        let hdr_map = HashMap::from([
            ("invalid?".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([
            ("invalid\0".to_owned(), "true".to_owned()),
            ("valid".to_owned(), "true".to_owned()),
        ]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([("User-Agent".to_string(), "true".to_owned())]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());

        let hdr_map = HashMap::from([("X-Amz-".to_string(), "true".to_owned())]);
        let endpoint_headers = EndpointHeaders(hdr_map);
        assert!(validate_header_map(&endpoint_headers.0).is_err());
    }

    #[test]
    fn test_endpoint_headers_patch_validation() {
        let hdr_map = HashMap::from([
            ("valid".to_owned(), Some("true".to_owned())),
            ("also-valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        endpoint_headers.validate().unwrap();

        let hdr_map = HashMap::from([
            ("invalid?".to_owned(), Some("true".to_owned())),
            ("valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([
            ("invalid\0".to_owned(), Some("true".to_owned())),
            ("valid".to_owned(), Some("true".to_owned())),
        ]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([("User-Agent".to_string(), Some("true".to_owned()))]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());

        let hdr_map = HashMap::from([("X-Amz-".to_string(), Some("true".to_owned()))]);
        let endpoint_headers = EndpointHeadersPatch(hdr_map);
        assert!(endpoint_headers.validate().is_err());
    }

    #[test]
    fn test_endpoint_secret_validation() {
        let secret = EndpointSecret::Symmetric(base64::decode("bm90LXZhbGlkCg==").unwrap());
        assert!(secret.validate().is_err());

        let secret =
            EndpointSecret::Symmetric(base64::decode("C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap());
        secret.validate().unwrap();

        let secret = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==").unwrap());
        secret.validate().unwrap();

        let secret = EndpointSecret::Asymmetric(AsymmetricKey::from_base64("6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlaaaaaaaaaaAJ6p9lMicMFs6Kvg==").unwrap());
        assert!(secret.validate().is_err());
    }

    #[derive(serde::Deserialize)]
    struct EndpointSecretTestStruct {
        key: EndpointSecret,
    }

    #[test]
    fn test_endpoint_secret_deserialization() {
        for key in [
            "w",
            "whsec_%",
            "whsec_wronglength",
            "whpk_1SiA4o9hyqTCpIqC5V9HUakiiaeACeqfZTInDBbOir4=", // Public key
            "whsk_6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kv", // Bad SK
            "hwsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD",
        ] {
            let js = serde_json::json!({ "key": key });
            assert!(serde_json::from_value::<EndpointSecretTestStruct>(js).is_err());
        }

        let js = serde_json::json!({ "key": "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD" });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Symmetric(key) = ep.key {
            assert_eq!(
                base64::decode("C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap(),
                key
            );
        } else {
            panic!("Shouldn't get here");
        }

        // Too long secret
        let js = serde_json::json!({ "key": "whsec_V09IYXZUaFJoSnFobnpJQkpPMXdpdGFNWnJsRzAxdXZCeTVndVpwRmxSSXFsc0oyYzBTRWRUekJhYnlaZ0JSRGNPQ3BGZG1xYjFVVmRGQ3UK" });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        assert!(ep.key.validate().is_err());

        // Valid long secret
        let long_sec = "TUdfVE5UMnZlci1TeWxOYXQtX1ZlTW1kLTRtMFdhYmEwanIxdHJvenRCbmlTQ2hFdzBnbHhFbWdFaTJLdzQwSA==";
        let js = serde_json::json!({ "key": format!("whsec_{long_sec}") });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Symmetric(key) = ep.key {
            assert_eq!(base64::decode(long_sec).unwrap(), key);
        } else {
            panic!("Shouldn't get here");
        }

        // Asymmetric key
        let asym_sec = "6Xb/dCcHpPea21PS1N9VY/NZW723CEc77N4rJCubMbfVKIDij2HKpMKkioLlX0dRqSKJp4AJ6p9lMicMFs6Kvg==";
        let js = serde_json::json!({ "key": format!("whsk_{asym_sec}") });
        let ep = serde_json::from_value::<EndpointSecretTestStruct>(js).unwrap();
        if let EndpointSecret::Asymmetric(key) = ep.key {
            assert_eq!(base64::decode(asym_sec).unwrap(), key.0.sk.as_slice());
        } else {
            panic!("Shouldn't get here");
        }
    }
}
