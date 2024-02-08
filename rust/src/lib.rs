// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Rust client library for Svix.
//!
//! The main entry points of this library are the API client [`api::Svix`], and
//! [`webhooks::Webhook`].

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use hyper::body::Bytes;
use hyper_util::client::legacy::{connect::HttpConnector, Client as HyperClient};

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_repr;

pub mod api;
pub mod error;
mod request;
pub mod webhooks;

#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
mod apis;
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
mod models;

struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: HyperClient<Connector, http_body_util::Full<Bytes>>,
    pub bearer_access_token: Option<String>,
}

// If no TLS backend is enabled, use plain http connector.
#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
type Connector = HttpConnector;

// If only native TLS is enabled, use that.
#[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
type Connector = hyper_tls::HttpsConnector<HttpConnector>;

// If rustls is enabled, use that.
#[cfg(feature = "rustls-tls")]
type Connector = hyper_rustls::HttpsConnector<HttpConnector>;

fn default_connector() -> Connector {
    #[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
    return hyper_util::client::legacy::connect::HttpConnector::new();

    #[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
    return hyper_tls::HttpsConnector::new();

    #[cfg(feature = "rustls-tls")]
    {
        let builder = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http();

        #[cfg(feature = "http1")]
        let builder = builder.enable_http1();

        #[cfg(feature = "http2")]
        let builder = builder.enable_http2();

        builder.build()
    }
}
