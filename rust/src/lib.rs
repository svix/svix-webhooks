// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_repr;

pub mod api;
pub mod error;
pub mod webhooks;

#[rustfmt::skip]
#[allow(dead_code)]
mod apis;
#[rustfmt::skip]
#[allow(dead_code)]
mod models;
