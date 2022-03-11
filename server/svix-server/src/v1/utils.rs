// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, error::Error as StdError, ops::Deref, str::FromStr};

use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, Query, RequestParts},
    BoxError, Json,
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use validator::Validate;

use crate::{
    core::types::{EventTypeName, EventTypeNameSet},
    error::{Error, HttpError, Result},
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

#[derive(Serialize)]
pub struct EmptyResponse {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T: Clone> {
    pub(crate) data: Vec<T>,
    pub(crate) iterator: Option<String>,
    pub(crate) done: bool,
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
        let Json(value) = Json::<T>::from_request(req)
            .await
            .map_err(|err| match err {
                JsonRejection::InvalidJsonBody(ref body_err) => HttpError::unprocessable_entity(
                    None,
                    Some(
                        body_err
                            .source()
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| err.to_string()),
                    ),
                ),
                _ => HttpError::bad_request(None, Some(err.to_string())),
            })?;
        value.validate().map_err(|x| {
            let message = format!("Input validation error: [{}]", x).replace('\n', ", ");
            HttpError::unprocessable_entity(None, Some(message))
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
        value.validate().map_err(|x| {
            let message = format!("Input validation error: [{}]", x).replace('\n', ", ");
            HttpError::unprocessable_entity(None, Some(message))
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
                    HttpError::unprocessable_entity(
                        None,
                        Some("Unable to parse before".to_string()),
                    )
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
