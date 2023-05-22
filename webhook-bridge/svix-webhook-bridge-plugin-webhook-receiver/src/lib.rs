use crate::config::IntegrationConfig;
use axum::{
    extract::{Path, State},
    routing::post,
    Router,
};
use forwarding::ForwardingMethod;
use serde::Deserialize;
use std::net::SocketAddr;
use svix_webhook_bridge_types::{
    async_trait, JsObject, JsReturn, Plugin, TransformerJob, TransformerTx,
};
use tracing::instrument;
use types::{IntegrationId, IntegrationState, InternalState, SerializableRequest, Unvalidated};

pub mod config;
mod forwarding;
mod types;
mod verification;

pub const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const PLUGIN_VERS: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct WebhookReceiverPluginConfig {
    pub listen_addr: SocketAddr,
    pub routes: Vec<IntegrationConfig>,
}

#[derive(Clone, Debug)]
pub struct WebhookReceiverPlugin {
    cfg: WebhookReceiverPluginConfig,
    transformer_tx: Option<TransformerTx>,
}

impl WebhookReceiverPlugin {
    pub fn new(cfg: WebhookReceiverPluginConfig) -> Self {
        Self {
            cfg,
            transformer_tx: None,
        }
    }
}

impl TryInto<Box<dyn Plugin>> for WebhookReceiverPluginConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn Plugin>, Self::Error> {
        Ok(Box::new(WebhookReceiverPlugin::new(self)))
    }
}

#[async_trait]
impl Plugin for WebhookReceiverPlugin {
    fn set_transformer(&mut self, tx: Option<TransformerTx>) {
        self.transformer_tx = tx;
    }

    async fn run(&self) -> std::io::Result<()> {
        let addr = &self.cfg.listen_addr;
        let state =
            InternalState::from_routes(self.cfg.routes.as_slice(), self.transformer_tx.clone())
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let router = Router::new()
            .route(
                "/webhook/:integration_id",
                post(route).put(route).get(route).patch(route),
            )
            .route(
                "/webhook/:integration_id/",
                post(route).put(route).get(route).patch(route),
            )
            .with_state(state);

        tracing::info!("Listening on: {addr}");
        axum::Server::bind(addr)
            .serve(router.into_make_service())
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

#[instrument(
    skip_all,
    level="error",
    fields(
        integration_id=integration_id.as_ref(),
        svixagent_plugin.name = PLUGIN_NAME,
        svixagent_plugin.vers = PLUGIN_VERS,
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
        forwarder,
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
                // `forward` method was ambiguous. It looks like there are many traits that offer
                // this method.
                match ForwardingMethod::forward(forwarder, payload).await {
                    Ok(c) => c,
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
    tx: Option<TransformerTx>,
) -> Result<JsObject, http::StatusCode> {
    let tx = tx.ok_or_else(|| {
        tracing::error!("transformations are not available");
        http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
