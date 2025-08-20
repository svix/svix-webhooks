// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Rust client library for Svix.
//!
//! The main entry points of this library are the API client [`api::Svix`], and
//! [`webhooks::Webhook`].

#![forbid(unsafe_code)]

use std::time::Duration;

use hyper::body::Bytes;
use hyper_util::client::legacy::Client as HyperClient;

pub mod api;
mod connector;
pub mod error;
mod model_ext;
mod models;
mod request;
pub mod webhooks;

pub(crate) use connector::{make_connector, Connector};

pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub bearer_access_token: Option<String>,
    pub timeout: Option<Duration>,
    pub num_retries: u32,
    pub retry_schedule: Option<Vec<Duration>>,

    client: HyperClient<Connector, http_body_util::Full<Bytes>>,
}

/// Convert a `StatusCode` from the http crate v1 to one from the http crate
/// v0.2.
fn http1_to_02_status_code(code: http1::StatusCode) -> http02::StatusCode {
    http02::StatusCode::from_u16(code.as_u16())
        .expect("both versions of the http crate enforce the same numerical limits for StatusCode")
}
