use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{Path, State},
    routing::post,
    Router,
};
use svix_bridge_types::{
    ForwardRequest, TransformationConfig, TransformerInput, TransformerInputFormat, TransformerJob,
    TransformerOutput, TransformerTx,
};
use tracing::instrument;
use types::{IntegrationId, IntegrationState, InternalState, SerializableRequest, Unvalidated};

use crate::{config::ReceiverConfig, webhook_receiver::types::SerializablePayload};

mod config;
mod types;
mod verification;

fn router() -> Router<InternalState, Body> {
    Router::new()
        .route(
            "/webhook/:integration_id",
            post(route).put(route).get(route).patch(route),
        )
        .route(
            "/webhook/:integration_id/",
            post(route).put(route).get(route).patch(route),
        )
}

pub async fn run(
    listen_addr: SocketAddr,
    routes: Vec<ReceiverConfig>,
    transformer_tx: TransformerTx,
) -> std::io::Result<()> {
    let state = InternalState::from_receiver_configs(routes, transformer_tx)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let router = router().with_state(state);

    tracing::info!("Listening on: {listen_addr}");
    axum::Server::bind(&listen_addr)
        .serve(router.into_make_service())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[instrument(
    skip_all,
    level = "error",
    fields(
        integration_id = integration_id.as_ref(),
    )
)]
async fn route(
    Path(integration_id): Path<IntegrationId>,
    State(InternalState {
        routes,
        transformer_tx,
    }): State<InternalState>,
    req: SerializableRequest<Unvalidated>,
) -> http::StatusCode {
    if let Some(IntegrationState {
        verifier,
        output,
        transformation,
    }) = routes.get(&integration_id)
    {
        match req.validate(verifier).await {
            Ok(req) => {
                let payload = match parse_payload(
                    req.payload(),
                    transformation,
                    transformer_tx.clone(),
                )
                .await
                {
                    Err(e) => return e,
                    Ok(p) => p,
                };
                tracing::debug!("forwarding request");
                match output.handle(payload).await {
                    Ok(_) => http::StatusCode::NO_CONTENT,
                    Err(e) => {
                        tracing::error!("Error forwarding request: {}", e);
                        http::StatusCode::INTERNAL_SERVER_ERROR
                    }
                }
            }
            Err(code) => {
                tracing::warn!("validation failed: {code}");
                code
            }
        }
    } else {
        tracing::trace!("integration not found");
        http::StatusCode::NOT_FOUND
    }
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
    transformation: &Option<TransformationConfig>,
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
                    TransformerInput::JSON(payload.as_json().map_err(|_| {
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

#[cfg(test)]
mod tests;
