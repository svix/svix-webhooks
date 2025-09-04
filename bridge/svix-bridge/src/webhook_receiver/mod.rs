use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    extract::{FromRequestParts, Path, State},
    http::{self, request},
    routing::{get, post},
    Router,
};
use svix_bridge_types::{
    async_trait,
    svix::{
        api::{MessagePollerConsumerPollOptions, PollingEndpointMessageOut, Svix},
        error::Error,
    },
    ForwardRequest, PollerInput, ReceiverOutput, TransformationConfig, TransformerInput,
    TransformerInputFormat, TransformerJob, TransformerOutput, TransformerTx,
};
use tracing::instrument;
use types::{IntegrationId, IntegrationState, InternalState, SerializableRequest, Unvalidated};

use crate::{
    config::{PollerInputOpts, PollerReceiverConfig, WebhookReceiverConfig},
    webhook_receiver::types::SerializablePayload,
};

mod config;
mod types;
mod verification;

fn router() -> Router<InternalState> {
    Router::new()
        .route(
            "/webhook/:integration_id",
            post(route).put(route).get(route).patch(route),
        )
        .route(
            "/webhook/:integration_id/",
            post(route).put(route).get(route).patch(route),
        )
        .route("/health", get(health_handler))
}
static START_TIME: once_cell::sync::Lazy<std::time::Instant> =
    once_cell::sync::Lazy::new(std::time::Instant::now);

fn get_uptime_seconds() -> u64 {
    START_TIME.elapsed().as_secs()
}
#[derive(serde::Serialize)]
struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime: u64,
}
async fn health_handler() -> impl axum::response::IntoResponse {
    let health_response = HealthResponse {
        status: "OK",
        version: env!("CARGO_PKG_VERSION"),
        uptime: get_uptime_seconds(),
    };
    axum::Json(health_response)
}
pub async fn run(
    listen_addr: SocketAddr,
    routes: Vec<WebhookReceiverConfig>,
    transformer_tx: TransformerTx,
) -> std::io::Result<()> {
    once_cell::sync::Lazy::force(&START_TIME);
    let state = InternalState::from_receiver_configs(routes, transformer_tx)
        .await
        .map_err(std::io::Error::other)?;

    let router = router().with_state(state);

    tracing::info!("Listening on: {listen_addr}");
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, router)
        .await
        .map_err(std::io::Error::other)
}

struct WebhookIdHeader(Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for WebhookIdHeader {
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(
            parts
                .headers
                .get("svix-id")
                .or_else(|| parts.headers.get("webhook-id"))
                .and_then(|val| Some(val.to_str().ok()?.to_owned())),
        ))
    }
}

#[instrument(
    skip_all,
    level = "error",
    fields(
        msg_id = _msg_id,
        integration_id = integration_id.as_ref(),
    )
)]
async fn route(
    Path(integration_id): Path<IntegrationId>,
    WebhookIdHeader(_msg_id): WebhookIdHeader,
    State(InternalState {
        routes,
        transformer_tx,
    }): State<InternalState>,
    req: SerializableRequest<Unvalidated>,
) -> Result<http::StatusCode, http::StatusCode> {
    let IntegrationState {
        verifier,
        output,
        transformation,
    } = routes
        .get(&integration_id)
        .ok_or(http::StatusCode::NOT_FOUND)?;

    let req = req.validate(verifier).await.inspect_err(|code| {
        tracing::warn!("validation failed: {code}");
    })?;

    let payload = parse_payload(
        req.payload(),
        transformation.as_ref(),
        transformer_tx.clone(),
    )
    .await?;

    handle(payload, Arc::clone(output)).await
}

// FIXME: Really odd return type - artifact of being extracted from the HTTP server
async fn handle(
    payload: ForwardRequest,
    output: Arc<Box<dyn ReceiverOutput>>,
) -> Result<http::StatusCode, http::StatusCode> {
    tracing::debug!("forwarding request");
    Ok(match output.handle(payload).await {
        Ok(_) => http::StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Error forwarding request: {}", e);
            http::StatusCode::INTERNAL_SERVER_ERROR
        }
    })
}

/// Figures out how to build a JSON object from the payload, optionally running it through a
/// transformation.
///
/// WRT "raw" payloads, the return value here is going to be a JSON object regardless of whether
/// or not the queue producer wants "raw" data.
///
/// When there's no transformation defined we therefore attempt to parse the body as json.
/// When a transformation is defined, we branch to see if it expects string or json input.
///
/// For either case, we expect the value produced to match the schema of a [`ForwardRequest`].
async fn parse_payload(
    payload: &SerializablePayload,
    transformation: Option<&TransformationConfig>,
    transformer_tx: TransformerTx,
) -> Result<ForwardRequest, http::StatusCode> {
    match transformation {
        Some(xform) => {
            let input = match xform.format() {
                TransformerInputFormat::String => {
                    TransformerInput::String(payload.as_string().map_err(|_| {
                        tracing::error!("Unable to parse request body as string");
                        http::StatusCode::BAD_REQUEST
                    })?)
                }
                TransformerInputFormat::Json => {
                    TransformerInput::Json(payload.as_json().map_err(|_| {
                        tracing::error!("Unable to parse request body as json");
                        http::StatusCode::BAD_REQUEST
                    })?)
                }
            };
            transform(input, xform.source().clone(), transformer_tx).await
        }
        // Keep the original payload as-is if there's no transformation specified, but stuff the
        // whole thing into the payload field.
        // The as_json() only gets us to `Value`, so we also need a `from_value` call to marshal
        // into a [`ForwardRequest`] type.
        None => Ok(ForwardRequest {
            payload: payload.as_json().map_err(|_| {
                tracing::error!("Unable to parse request body as json");
                http::StatusCode::BAD_REQUEST
            })?,
        }),
    }
}

/// Attempts to run the payload through a js transformation.
async fn transform(
    input: TransformerInput,
    script: String,
    tx: TransformerTx,
) -> Result<ForwardRequest, http::StatusCode> {
    let (job, callback) = TransformerJob::new(script, input);
    if let Err(e) = tx.send(job) {
        tracing::error!("transformations are not available: {}", e);
        return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    match callback.await {
        // This is the only "good" outcome giving a RHS value for the assignment.
        // All other match arms should bail with a non-2xx status.
        Ok(Ok(TransformerOutput::Object(obj))) => Ok(serde_json::from_value(
            serde_json::Value::Object(obj),
        )
        .map_err(|e| {
            tracing::error!("transformation produced invalid payload: {}", e);
            http::StatusCode::INTERNAL_SERVER_ERROR
        })?),
        Ok(Ok(TransformerOutput::Invalid)) => {
            tracing::error!("transformation produced invalid payload");
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
        _ => {
            tracing::error!("transformation failed");
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

struct SvixEventsPoller {
    name: String,
    input_opts: PollerInputOpts,
    transformation: Option<TransformationConfig>,
    transformer_tx: Option<TransformerTx>,
    svix_client: Svix,
    output: Arc<Box<dyn ReceiverOutput>>,
}

#[async_trait]
impl PollerInput for SvixEventsPoller {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }

    async fn run(&self) {
        run_inner(self).await
    }
}

impl PollerReceiverConfig {
    pub async fn into_poller_input(
        self,
        transformer_tx: TransformerTx,
    ) -> std::io::Result<Box<dyn PollerInput>> {
        let svix_client = self
            .input
            .svix_client()
            .expect("only one poller type; svix client required");
        let name = self.name.clone();
        let input_opts = self.input.clone();
        let transformation = self.transformation.clone();
        let output = Arc::new(
            self.into_receiver_output()
                .await
                .map_err(std::io::Error::other)?,
        );
        Ok(Box::new(SvixEventsPoller {
            name,
            input_opts,
            transformation,
            transformer_tx: Some(transformer_tx.clone()),
            svix_client,
            output,
        }))
    }
}

async fn run_inner(poller: &SvixEventsPoller) -> ! {
    const MIN_SLEEP: Duration = Duration::from_millis(10);
    const MAX_SLEEP: Duration = Duration::from_secs(300);
    const NO_SLEEP: Duration = Duration::ZERO;
    let mut sleep_time = NO_SLEEP;

    let PollerInputOpts::SvixMessagePoller {
        consumer_id,
        token: _,
        app_id,
        sink_id,
        svix_options: _,
    } = &poller.input_opts;

    let mut iterator = None;

    'outer: loop {
        tracing::trace!(app_id, sink_id, "polling poller");
        match poller
            .svix_client
            .message()
            .poller()
            .consumer_poll(
                app_id.clone(),
                sink_id.clone(),
                consumer_id.clone(),
                Some(MessagePollerConsumerPollOptions {
                    limit: None,
                    iterator: iterator.clone(),
                }),
            )
            .await
        {
            Ok(resp) => {
                let mut has_failure = false;
                tracing::trace!(count = resp.data.len(), "got messages");
                for msg in resp.data {
                    let msg_id = msg.id.clone();
                    if let Err((status, message)) = handle_poller_msg(msg, poller).await {
                        tracing::error!(msg_id, status, message);
                        has_failure = true;
                    }
                }

                // Retry the current iterator if we see failures while handling any of the messages
                // in the batch.
                if has_failure {
                    // BACKOFF
                    sleep_time = (sleep_time * 2).clamp(MIN_SLEEP, MAX_SLEEP);
                } else {
                    tracing::trace!(
                        ?iterator,
                        next_iterator = ?resp.iterator,
                        "batch handled, updating local iterator"
                    );
                    // Update the iterator _after we've handled all the messages in the batch_.
                    iterator = Some(resp.iterator.clone());
                    // If the iterator is "done" we can backoff to wait for new messages to arrive.
                    sleep_time = if resp.done {
                        // BACKOFF
                        (sleep_time * 2).clamp(MIN_SLEEP, MAX_SLEEP)
                    } else {
                        NO_SLEEP
                    };
                }
            }

            Err(err) => {
                match &err {
                    Error::Http(x)
                        if x.status.as_u16() == http::StatusCode::BAD_REQUEST.as_u16() =>
                    {
                        if let Some("invalid_iterator") =
                            x.payload.as_ref().map(|p| p.code.as_str())
                        {
                            tracing::error!(
                                error = ?err,
                                ?iterator,
                                "request failed, iterator is invalid syncing with server..."
                            );
                            iterator = None;
                            continue 'outer;
                        }
                    }
                    _ => {}
                }

                tracing::error!(
                    error = ?err,
                    ?iterator,
                    "request failed, retrying current iterator"
                );
                // BACKOFF
                sleep_time = (sleep_time * 2).clamp(MIN_SLEEP, MAX_SLEEP);
            }
        }

        if !sleep_time.is_zero() {
            tracing::trace!(?sleep_time, "sleeping");
            tokio::time::sleep(sleep_time).await;
        }
    }
}

#[tracing::instrument(skip_all, fields(msg_id = msg.id))]
async fn handle_poller_msg(
    msg: PollingEndpointMessageOut,
    poller: &SvixEventsPoller,
) -> Result<(), (u16, &'static str)> {
    let payload = parse_payload(
        &SerializablePayload::Standard(
            // FIXME: for svix-event pollers we already know the payload is json so
            //   there's some wasted ser/deser/ser cycles.
            serde_json::to_vec(&msg).expect("just fetched as json, must be serializable"),
        ),
        poller.transformation.as_ref(),
        poller
            .transformer_tx
            .clone()
            .expect("transformer tx is required"),
    )
    .await
    .map_err(|status| (status.as_u16(), "error while parsing polled message"))?;

    handle(payload, Arc::clone(&poller.output))
        .await
        // FIXME: need to refactor handle to not give http status codes so we can report what happened here.
        .map_err(|status| (status.as_u16(), "error while handling polled message"))?;

    Ok(())
}

#[cfg(test)]
mod tests;
