//! Extensions of the auto-generated "models" (schema structs).

use serde_json::json;

use crate::models::MessageIn;

impl MessageIn {
    /// Create a new message with a pre-serialized payload.
    ///
    /// The payload is not normalized on the server (usually whitespace outside
    /// of string literals, unnecessarily escaped characters in string and such
    /// are fixed up by the server), and is not even required to be JSON.
    ///
    /// The default `content-type` of `application/json` will still be used,
    /// unless overwritten with [`with_content_type`][Self::with_content_type].
    pub fn new_raw_payload(event_type: String, payload: String) -> Self {
        Self {
            transformations_params: Some(json!({ "rawPayload": payload })),
            ..Self::new(event_type, json!({}))
        }
    }

    /// Set the `content-type` header to use for the message.
    pub fn with_content_type(mut self, content_type: String) -> Self {
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
