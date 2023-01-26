// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Module defining utilites for PATCH requests focused mostly around non-required field types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// This is an enum that will wrap every nullable field for a PATCH request. Nonnullable fields can
/// be represented via an [`UnrequiredField`]. This differs from an [`Option`] in that it
/// distinguishes null values and absent values such that an optional value in a model may be made
/// None via PATCHing while allowing omitted fields to be skipped when updating.
///
/// NOTE: You must tag these fields with `#[serde(default)]` in order for the serialization to work
/// correctly.
#[derive(Debug)]
pub enum UnrequiredNullableField<T> {
    Absent,
    None,
    Some(T),
}

/// This enum is a non-nullable equivalent to [`UnrequiredNullableField`]. This is effectively an
/// [`Option`] with the additional context that any field which uses this type is a member of a
/// PATCH request model and that the field may be absent, meaning it is not to be updated. In
/// comparison, [`Option`]s are used in other [`ModelIn`]s to define a field, that when absent,
/// is `null`.
///
/// NOTE: You must tag these fields with `#[serde(default)]` in order for the serialization to work
/// correctly.
#[derive(Debug)]
pub enum UnrequiredField<T> {
    Absent,
    Some(T),
}

impl<T> UnrequiredNullableField<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, UnrequiredNullableField::Absent)
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> UnrequiredNullableField<U> {
        match self {
            UnrequiredNullableField::Absent => UnrequiredNullableField::Absent,
            UnrequiredNullableField::None => UnrequiredNullableField::None,
            UnrequiredNullableField::Some(v) => UnrequiredNullableField::Some(f(v)),
        }
    }
}

impl<T> UnrequiredField<T> {
    pub fn is_absent(&self) -> bool {
        matches!(self, UnrequiredField::Absent)
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> UnrequiredField<U> {
        match self {
            UnrequiredField::Absent => UnrequiredField::Absent,
            UnrequiredField::Some(v) => UnrequiredField::Some(f(v)),
        }
    }
}

impl<T> Default for UnrequiredNullableField<T> {
    fn default() -> Self {
        Self::Absent
    }
}

impl<T> Default for UnrequiredField<T> {
    fn default() -> Self {
        Self::Absent
    }
}

impl<T> From<Option<T>> for UnrequiredNullableField<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(v) => UnrequiredNullableField::Some(v),
            None => UnrequiredNullableField::None,
        }
    }
}

impl<T: Validate> Validate for UnrequiredNullableField<T> {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            UnrequiredNullableField::Absent | UnrequiredNullableField::None => Ok(()),
            UnrequiredNullableField::Some(v) => v.validate(),
        }
    }
}

impl<T: Validate> Validate for UnrequiredField<T> {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            UnrequiredField::Absent => Ok(()),
            UnrequiredField::Some(v) => v.validate(),
        }
    }
}

impl<T: Clone> Clone for UnrequiredNullableField<T> {
    fn clone(&self) -> Self {
        match self {
            UnrequiredNullableField::Absent => UnrequiredNullableField::Absent,
            UnrequiredNullableField::None => UnrequiredNullableField::None,
            UnrequiredNullableField::Some(v) => UnrequiredNullableField::Some(v.clone()),
        }
    }
}

impl<T: Clone> Clone for UnrequiredField<T> {
    fn clone(&self) -> Self {
        match self {
            UnrequiredField::Absent => UnrequiredField::Absent,
            UnrequiredField::Some(v) => UnrequiredField::Some(v.clone()),
        }
    }
}

impl<T: Clone + Copy> Copy for UnrequiredNullableField<T> {}
impl<T: Clone + Copy> Copy for UnrequiredField<T> {}

impl<'de, T> Deserialize<'de> for UnrequiredNullableField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<'de, T> Deserialize<'de> for UnrequiredField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(UnrequiredField::Some)
    }
}

impl<T> Serialize for UnrequiredNullableField<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UnrequiredNullableField::Absent => Err(serde::ser::Error::custom(
                "UnrequiredNullableField must skip serializing if field is absent",
            )),
            UnrequiredNullableField::None => serializer.serialize_none(),
            UnrequiredNullableField::Some(v) => v.serialize(serializer),
        }
    }
}
impl<T> Serialize for UnrequiredField<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            UnrequiredField::Absent => Err(serde::ser::Error::custom(
                "UnrequiredField must skip serializing if field is absent",
            )),
            UnrequiredField::Some(v) => v.serialize(serializer),
        }
    }
}

impl<T: JsonSchema> JsonSchema for UnrequiredField<T> {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        format!("Unrequired_{}", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Option::<T>::json_schema(gen)
    }
}

impl<T: JsonSchema> JsonSchema for UnrequiredNullableField<T> {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        format!("UnrequiredNullable_{}", T::schema_name())
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        Option::<T>::json_schema(gen)
    }
}

/// Macro that simplifies updating a field on an [`ActiveModel`] for use in a [`ModelIn`]
/// implementation. This macro expands to setting the field when the [`Option`] is `Some`, but
/// performs no operation in the case it is `None`.
///
/// The input for this macro is three identifiers meant to be `self`, the `model` in a [`ModelIn`]
/// implementation, and the member that `self`, and `model` share that is being modified.
///
/// Optionally, a fourth identifier may be given which is meant to be a closure that takes the type
/// of self's version of the member beng modified and returns model's version of the member being
/// modified. This is applied via [`UnrequiredNullableField::map`] such that  basic type conversions may
/// be made.
///
/// The nullable equivalent which is used for [`UnrequiredNullableField`] is [`patch_field_nullable`].
macro_rules! patch_field_non_nullable {
    ($model:ident, $member:ident) => {
        match $member {
            UnrequiredField::Some(v) => $model.$member = Set(v),
            UnrequiredField::Absent => {}
        }
    };

    ($model:ident, $member:ident, $f:ident) => {
        let mapped = $member.map($f);
        match mapped {
            UnrequiredField::Some(v) => $model.$member = Set(v),
            UnrequiredField::Absent => {}
        }
    };
}
pub(crate) use patch_field_non_nullable;

/// Macro that simplifies updating a field on an [`ActiveModel`] for use in a [`ModelIn`]
/// implementation. This macro expands to setting the field when the [`UnrequiredNullableField`] is
/// `Some` and unsetting the field when it is `None`, but performs no operation in the case it is
///  `Absent`.
///
/// The input for this macro is three identifiers meant to be `self`, the `model` in a [`ModelIn`]
/// implementation, and the member that `self`, and `model` share that is being modified.
///
/// Optionally, a fourth identifier may be given which is meant to be a closure that takes the type
/// of self's version of the member beng modified and returns model's version of the member being
/// modified. This is applied via [`UnrequiredNullableField::map`] such that  basic type conversions may
/// be made.
///
/// The non-nullable equivalent which is used for [`Option`] is [`patch_field_non_nullable`].
macro_rules! patch_field_nullable {
    ($model:ident, $member:ident) => {
        match $member {
            UnrequiredNullableField::Some(v) => $model.$member = Set(Some(v)),
            UnrequiredNullableField::None => $model.$member = Set(None),
            UnrequiredNullableField::Absent => {}
        }
    };

    ($model:ident, $member:ident, $f:ident) => {
        let mapped = $member.map($f);
        match mapped {
            UnrequiredNullableField::Some(v) => $model.$member = Set(Some(v)),
            UnrequiredNullableField::None => $model.$member = Set(None),
            UnrequiredNullableField::Absent => {}
        }
    };
}
pub(crate) use patch_field_nullable;
