// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Module defining utilities for crating `tracing` spans compatible with OpenTelemetry's
//! conventions.
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, MatchedPath};
use http::header;
use opentelemetry::trace::TraceContextExt;
use svix_ksuid::{KsuidLike, KsuidMs};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{MakeSpan, OnFailure, OnResponse},
};
use tracing::field::{debug, Empty};
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
            .and_then(|header| header.to_str().ok());

        let host = request
            .headers()
            .get(header::HOST)
            .and_then(|header| header.to_str().ok());

        let http_route = request
            .extensions()
            .get::<MatchedPath>()
            .map(|p| p.as_str());

        let client_ip = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(ip)| debug(ip));

        let request_id = request
            .headers()
            .get("x-request-id")
            .and_then(|id| id.to_str().map(ToOwned::to_owned).ok())
            // If `x-request-id` isn't set, check `svix-req-id`. If the `svix-req-id` isn't a
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
            .then(|| span_context.trace_id().to_string());

        let idempotency_key = request
            .headers()
            .get("idempotency-key")
            .and_then(|v| v.to_str().ok());

        let span = tracing::error_span!(
            "HTTP request",
            grpc.code = Empty,
            http.client_ip = client_ip,
            http.versions = ?request.version(),
            http.host = host,
            http.method = ?request.method(),
            http.route = http_route,
            http.scheme = request.uri().scheme().map(debug),
            http.status_code = Empty,
            http.target = request.uri().path_and_query().map(|p| p.as_str()),
            http.user_agent = user_agent,
            otel.kind = "server",
            otel.status_code = Empty,
            request_id,
            trace_id,
            idempotency_key,
            org_id = tracing::field::Empty,
            app_id = tracing::field::Empty,
        );

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
