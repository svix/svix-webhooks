// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Rust client library for Svix.
//!
//! The main entry points of this library are the API client [`api::Svix`], and
//! [`webhooks::Webhook`].

#![forbid(unsafe_code)]

pub mod api;
mod api_internal;
pub mod autoconfig;
pub mod autoconfig_consumer;
mod connector;
pub mod error;
mod model_ext;
pub mod models;
mod request;
pub mod webhooks;

pub(crate) use api::client::Configuration;
