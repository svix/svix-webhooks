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
mod api_internal;
pub mod autoconfig;
pub mod autoconfig_consumer;
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
