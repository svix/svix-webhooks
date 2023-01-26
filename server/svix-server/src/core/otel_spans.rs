// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Module defining utilities for crating `tracing` spans compatable with OpenTelemetry's
//! conventions.
use std::{borrow::Cow, net::SocketAddr};

use axum::extract::{ConnectInfo, MatchedPath};
use http::{header, uri::Scheme, Method, Version};
use opentelemetry::trace::TraceContextExt;
use svix_ksuid::{KsuidLike, KsuidMs};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{MakeSpan, OnFailure, OnResponse},
};
use tracing::field::Empty;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// An implementor of [`MakeSpan`] which creates `tracing` spans populated with information about
/// the request received by an `axum` web server.
#[derive(Clone, Copy)]
pub struct AxumOtelSpanCreator;

impl<B> MakeSpan<B> for AxumOtelSpanCreator {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        let user_agent = request
            .headers()
            .get(header::USER_AGENT)
            .map_or("", |header| header.to_str().unwrap_or(""));

        let host = request
            .headers()
            .get(header::HOST)
            .map_or("", |header| header.to_str().unwrap_or(""));

        let scheme = request.uri().scheme().map_or_else(
            || "HTTP".into(),
            |scheme| -> Cow<'static, str> {
                if scheme == &Scheme::HTTP {
                    "http".into()
                } else if scheme == &Scheme::HTTPS {
                    "https".into()
                } else {
                    scheme.to_string().into()
                }
            },
        );

        let http_route = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_owned()
        } else {
            request.uri().path().to_owned()
        };

        let client_ip: Cow<'static, str> = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(ip)| Cow::from(ip.to_string()))
            .unwrap_or_default();

        let request_id = request
            .headers()
            .get("x-request-id")
            .and_then(|id| id.to_str().map(ToOwned::to_owned).ok())
            // If `x-requst-id` isn't set, check `svix-req-id`. If the `svix-req-id` isn't a
            // valid `str`, or it isn't set, then fallback to a random [`KsuidMs`]
            .or_else(|| {
                request
                    .headers()
                    .get("svix-req-id")
                    .and_then(|v| v.to_str().map(ToOwned::to_owned).ok())
            })
            .unwrap_or_else(|| KsuidMs::new(None, None).to_string());

        let remote_context = opentelemetry::global::get_text_map_propagator(|p| {
            p.extract(&opentelemetry_http::HeaderExtractor(request.headers()))
        });
        let remote_span = remote_context.span();
        let span_context = remote_span.span_context();
        let trace_id = span_context
            .is_valid()
            .then(|| Cow::from(span_context.trace_id().to_string()))
            .unwrap_or_default();

        let flavor: Cow<'static, str> = match request.version() {
            Version::HTTP_09 => "0.9".into(),
            Version::HTTP_10 => "1.0".into(),
            Version::HTTP_11 => "1.1".into(),
            Version::HTTP_2 => "2.0".into(),
            Version::HTTP_3 => "3.0".into(),
            other => format!("{other:?}").into(),
        };

        let method: Cow<'static, str> = match request.method() {
            &Method::CONNECT => "CONNECT".into(),
            &Method::DELETE => "DELETE".into(),
            &Method::GET => "GET".into(),
            &Method::OPTIONS => "OPTIONS".into(),
            &Method::PATCH => "PATCH".into(),
            &Method::POST => "POST".into(),
            &Method::PUT => "PUT".into(),
            &Method::TRACE => "TRACE".into(),
            other => other.to_string().into(),
        };

        let idempotency_key = request
            .headers()
            .get("idempotency-key")
            .and_then(|v| v.to_str().ok());

        let span = tracing::error_span!(
            "HTTP request",
            grpc.code = Empty,
            http.client_ip = %client_ip,
            http.flavor = %flavor,
            http.host = %host,
            http.method = %method,
            http.route = %http_route,
            http.scheme = %scheme,
            http.status_code = Empty,
            http.target = %request.uri().path_and_query().map_or("", |p| p.as_str()),
            http.user_agent = %user_agent,
            otel.kind = "server",
            otel.status_code = Empty,
            request_id = %request_id,
            trace_id = %trace_id,
            idempotency_key = tracing::field::Empty,
            org_id = tracing::field::Empty,
            app_id = tracing::field::Empty,
        );

        if let Some(key) = idempotency_key {
            span.record("idempotency_key", key);
        }

        span.set_parent(remote_context);

        span
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AxumOtelOnResponse;

impl<B> OnResponse<B> for AxumOtelOnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let status = response.status().as_u16().to_string();
        span.record("http.status_code", &tracing::field::display(status));
        span.record("otel.status_code", "OK");

        tracing::debug!(
            "finished processing request latency={} ms status={}",
            latency.as_millis(),
            response.status().as_u16(),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AxumOtelOnFailure;

impl OnFailure<ServerErrorsFailureClass> for AxumOtelOnFailure {
    fn on_failure(
        &mut self,
        failure_classification: ServerErrorsFailureClass,
        _latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        match failure_classification {
            ServerErrorsFailureClass::StatusCode(status) if status.is_server_error() => {
                span.record("otel.status_code", "ERROR");
            }
            _ => {}
        }
    }
}
