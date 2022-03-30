// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{borrow::Cow, collections::HashSet, error::Error as StdError, ops::Deref, str::FromStr};

use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Query, RequestParts},
    BoxError,
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use validator::Validate;

use crate::{
    core::types::{EventTypeName, EventTypeNameSet},
    error::{Error, HttpError, Result, ValidationErrorItem},
};

const fn default_limit() -> u64 {
    50
}

#[derive(Debug, Deserialize, Validate)]
pub struct Pagination<T: Validate> {
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[validate]
    pub iterator: Option<T>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReversablePagination<T: 'static + Validate + From<String>> {
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[validate]
    pub iterator: Option<PaginationIterator<T>>,
}

#[derive(Debug, PartialEq)]
pub enum PaginationIterator<T: Validate> {
    Normal(T),
    Prev(T),
}

impl<'de, T: 'static + Deserialize<'de> + Validate + From<String>> Deserialize<'de>
    for PaginationIterator<T>
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor(std::marker::PhantomData))
    }
}

impl<T: Validate> Validate for PaginationIterator<T> {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            PaginationIterator::Normal(val) => val.validate(),
            PaginationIterator::Prev(val) => val.validate(),
        }
    }
}

struct Visitor<'de, T: Deserialize<'de> + Validate>(std::marker::PhantomData<fn() -> &'de T>);

impl<'de, T: Deserialize<'de> + Validate + From<String>> serde::de::Visitor<'de>
    for Visitor<'de, T>
{
    type Value = PaginationIterator<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an iterator value")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with('-') {
            let v = v.trim_start_matches('-');
            Ok(PaginationIterator::Prev(T::from(v.to_owned())))
        } else {
            Ok(PaginationIterator::Normal(T::from(v)))
        }
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with('-') {
            let v = v.trim_start_matches('-');
            Ok(PaginationIterator::Prev(T::from(v.to_owned())))
        } else {
            Ok(PaginationIterator::Normal(T::from(v.to_owned())))
        }
    }
}

#[derive(Serialize)]
pub struct EmptyResponse {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T: Clone> {
    pub data: Vec<T>,
    pub iterator: Option<String>,
    pub done: bool,
}

pub trait ModelIn {
    type ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel);
}

pub trait ModelOut {
    fn id_copy(&self) -> String;

    fn list_response<T: ModelOut + Clone>(mut data: Vec<T>, limit: usize) -> ListResponse<T> {
        let done = data.len() <= limit;
        data.truncate(limit);
        let iterator = data.last().map(|x| x.id_copy());
        ListResponse {
            data,
            iterator,
            done,
        }
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
                        loc.push(format!("[{}]", k));

                        validation_errors(loc, *v)
                    })
                    .collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let b = bytes::Bytes::from_request(req).await.map_err(|e| {
            tracing::error!("Error reading body as bytes: {}", e);
            HttpError::internal_server_errer(None, Some("Failed to read request body".to_owned()))
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
impl<T, B> FromRequest<B> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let Query(value) = Query::<T>::from_request(req)
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

/// This struct is slower than Query. Only use this if we need to pass arrays.
#[derive(Debug)]
pub struct MessageListFetchOptions {
    pub event_types: Option<EventTypeNameSet>,
    pub before: Option<DateTime<Utc>>,
}

#[async_trait]
impl<B> FromRequest<B> for MessageListFetchOptions
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
        let pairs: Vec<(String, String)> =
            serde_urlencoded::from_str(req.uri().query().unwrap_or_default())
                .map_err(|err| HttpError::bad_request(None, Some(err.to_string())))?;

        let mut before = None;
        let mut event_types = EventTypeNameSet(HashSet::<EventTypeName>::new());
        for (key, value) in pairs {
            if key == "event_types" {
                event_types.0.insert(EventTypeName(value));
            } else if key == "before" {
                before = Some(DateTime::<Utc>::from_str(&value).map_err(|_| {
                    HttpError::unprocessable_entity(vec![ValidationErrorItem {
                        loc: vec!["query".to_owned(), "before".to_owned()],
                        msg: "Unable to parse before".to_owned(),
                        ty: "value_error".to_owned(),
                    }])
                })?);
            }
        }
        let event_types = if event_types.0.is_empty() {
            None
        } else {
            Some(event_types)
        };
        Ok(MessageListFetchOptions {
            event_types,
            before,
        })
    }
}

pub async fn api_not_implemented() -> Result<()> {
    Err(HttpError::not_implemented(None, None).into())
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::{default_limit, validation_errors, Pagination};
    use crate::core::types::ApplicationUid;
    use crate::error::ValidationErrorItem;
    use serde_json::json;

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
        assert_eq!(p.limit, default_limit());
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
        iterator: super::PaginationIterator<crate::core::types::MessageId>,
    }

    #[test]
    fn test_pagination_deserialization() {
        let a = serde_json::json!({"iterator": "msg_274DTsX0wVTSLvo91QopQgZrjDV"});
        let b = serde_json::json!({"iterator": "-msg_274DTsX0wVTSLvo91QopQgZrjDV"});

        assert_eq!(
            serde_json::from_value::<TestPaginationDeserializationStruct>(a).unwrap(),
            TestPaginationDeserializationStruct {
                iterator: super::PaginationIterator::Normal(crate::core::types::MessageId(
                    "msg_274DTsX0wVTSLvo91QopQgZrjDV".to_owned()
                ))
            }
        );
        assert_eq!(
            serde_json::from_value::<TestPaginationDeserializationStruct>(b).unwrap(),
            TestPaginationDeserializationStruct {
                iterator: super::PaginationIterator::Prev(crate::core::types::MessageId(
                    "msg_274DTsX0wVTSLvo91QopQgZrjDV".to_owned()
                ))
            }
        );
    }
}
