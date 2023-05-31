use crate::config::ReceiverConfig;
use axum::{
    body::Body,
    extract::{Path, State},
    routing::post,
    Router,
};
use std::net::SocketAddr;
use svix_bridge_types::{JsObject, JsReturn, TransformerJob, TransformerTx};
use tracing::instrument;
use types::{IntegrationId, IntegrationState, InternalState, SerializableRequest, Unvalidated};

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
    level="error",
    fields(
        integration_id=integration_id.as_ref(),
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
                let payload = match req.payload().as_js_object() {
                    Ok(payload) => match transformation.clone() {
                        Some(script) => {
                            match transform(payload, script, transformer_tx.clone()).await {
                                Ok(transformed_payload) => transformed_payload,
                                Err(c) => return c,
                            }
                        }
                        // Keep the original payload as-is if there's no transformation specified.
                        None => payload,
                    },
                    Err(e) => {
                        tracing::error!("failed to parse payload as json object: {}", e);
                        return http::StatusCode::BAD_REQUEST;
                    }
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

/// Attempts to run the payload through a js transformation.
async fn transform(
    payload: JsObject,
    script: String,
    tx: TransformerTx,
) -> Result<JsObject, http::StatusCode> {
    let (job, callback) = TransformerJob::new(script.clone(), payload);
    if let Err(e) = tx.send(job) {
        tracing::error!("transformations are not available: {}", e);
        return Err(http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    match callback.await {
        // This is the only "good" outcome giving a RHS value for the assignment.
        // All other match arms should bail with a non-2xx status.
        Ok(Ok(JsReturn::Object(obj))) => Ok(obj),
        Ok(Ok(JsReturn::Invalid)) => {
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
