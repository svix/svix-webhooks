use serde::Deserialize;
use svix_bridge_types::{TransformationConfig, WebhookVerifier};

use crate::config::ReceiverConfig;

/// The [`IntegrationConfig`] is the struct associated with a given [`IntegrationId`]. When the route
/// associated with an [`IntegrationId`] receives a webhook, or any other HTTP request, then it will
/// attempt to be validated with the specified [`VerificationScheme`]. Should the configured scheme
/// indicate that the webhook is valid, then the webhook will be forwarded verbatim to the configured
/// [`ForwardDestination`].
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct IntegrationConfig {
    pub receiver_cfg: ReceiverConfig,
    pub verification: WebhookVerifier,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
}
