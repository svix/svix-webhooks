//! Extensions of the auto-generated "models" (schema structs).

use std::str::FromStr;

use serde_json::json;

use crate::{
    api::{
        MessageStatus,
        Ordering,
        StatusCodeClass,
    },
    models::MessageIn,
};

impl MessageIn {
    /// Create a new message with a raw string payload.
    ///
    /// The payload is not normalized on the server. Normally, payloads are
    /// required to be JSON, and Svix will minify the payload before sending
    /// the webhook (for example, by removing extraneous whitespace or
    /// unnecessarily escaped characters in strings). With this constructor,
    /// the payload will be sent "as is", without any minification or other
    /// processing.
    ///
    /// The default `content-type` of `application/json` will still be used for
    /// the webhook sent by Svix, unless overwritten with
    /// [`with_content_type`][Self::with_content_type].
    pub fn new_raw_payload(
        event_type: String,
        payload: String,
    ) -> Self {
        Self {
            transformations_params: Some(json!({ "rawPayload": payload })),
            ..Self::new(
                event_type,
                json!({}),
            )
        }
    }

    /// Set the `content-type` header to use for the webhook sent by Svix.
    pub fn with_content_type(
        mut self,
        content_type: String,
    ) -> Self {
        let transformations_params = self.transformations_params.get_or_insert_with(|| json!({}));

        // This will panic if transformations_params, its headers field, or the
        // headers' content-type field already exists as an array, bool, number
        // or string.
        // That would make the whole parameter struct invalid anyways though,
        // and can hardly happen accidentally.
        transformations_params["headers"]["content-type"] = content_type.into();

        self
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid value for ordering")]
pub struct OrderingFromStrError;

impl FromStr for Ordering {
    type Err = OrderingFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascending" => Ok(Self::Ascending),
            "descending" => Ok(Self::Descending),
            _ => Err(OrderingFromStrError),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid value for messageStatus")]
pub struct MessageStatusFromStrError;

impl FromStr for MessageStatus {
    type Err = MessageStatusFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "success" => Ok(Self::Success),
            "1" | "pending" => Ok(Self::Pending),
            "2" | "fail" => Ok(Self::Fail),
            "3" | "sending" => Ok(Self::Sending),
            _ => Err(MessageStatusFromStrError),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid value for statusCodeClass")]
pub struct StatusCodeClassFromStrError;

impl FromStr for StatusCodeClass {
    type Err = StatusCodeClassFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::CodeNone),
            "100" => Ok(Self::Code1xx),
            "200" => Ok(Self::Code2xx),
            "300" => Ok(Self::Code3xx),
            "400" => Ok(Self::Code4xx),
            "500" => Ok(Self::Code5xx),
            _ => Err(StatusCodeClassFromStrError),
        }
    }
}
