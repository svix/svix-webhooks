// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

pub mod cache;
pub mod cryptography;
pub mod idempotency;
pub mod message_app;
pub mod operational_webhooks;
pub mod otel_spans;
pub mod permissions;
pub mod run_with_retries;
pub mod security;
pub mod types;
pub mod webhook_http_client;

#[cfg(test)]
mod tests {
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    // demonstrates that JSON serialized by serde_json is in "compact-mode"
    // i.e. that it has no spaces and is comparable to JSON.stringify in Javascript
    #[test]
    fn test_json_is_compact() {
        let test_struct = TestStruct {
            field1: String::from("test"),
            field2: 42,
        };

        let serialized_json = serde_json::to_string(&test_struct).unwrap();
        let desired_json = r#"{"field1":"test","field2":42}"#;

        assert_eq!(serialized_json, desired_json);
    }
}
