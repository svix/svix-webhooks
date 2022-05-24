// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod api;
pub mod error;
pub mod webhooks;

#[rustfmt::skip]
#[allow(dead_code)]
mod apis;
#[rustfmt::skip]
#[allow(dead_code)]
mod models;
