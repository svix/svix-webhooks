use crate::{
    api::{EndpointIn, EndpointOut, Svix, SvixOptions},
    error::Result,
    internal::{api::InternalApi, models::subscribe_in::SubscribeIn},
    webhooks::{HeaderMap, Webhook, WebhookError},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Content of the auto-config token. Fields are renamed to shorten the length of final base64 encoded token.
struct AutoConfigTokenContentV1 {
    /// The endpoint ID.
    #[serde(rename = "eid")]
    pub endpoint_id: String,
    /// Server URL.
    #[serde(rename = "surl")]
    pub server_url: String,
    /// The endpoint secret. To verify incoming webhooks from an endpoint created with this subscription.
    #[serde(rename = "esec")]
    pub endpoint_secret: String,
    /// The token plaintext. To be used to call complete_subscription.
    #[serde(rename = "tok")]
    // Note: It's important that this field is last in this struct, so the censored token always contains part of the revealed token,
    // since that's the only way to differentiate between different magic tokens for the same endpoint.
    pub token_plaintext: String,
}

pub struct AutoConfig {
    endpoint: EndpointIn,
    webhook: Webhook,
    endpoint_id: String,
    svix: Svix,
}

const AUTOCONFIG_TOKEN_PREFIX_V1: &str = "auto_v1_";

#[derive(thiserror::Error, Debug)]
pub enum AutoConfigError {
    #[error("invalid token")]
    InvalidToken,
}

impl AutoConfig {
    pub fn new(token: String, endpoint: EndpointIn) -> std::result::Result<Self, AutoConfigError> {
        let token = token
            .strip_prefix(AUTOCONFIG_TOKEN_PREFIX_V1)
            .ok_or(AutoConfigError::InvalidToken)?;

        let content = base64::decode(token).map_err(|_| AutoConfigError::InvalidToken)?;
        let content = serde_json::from_slice::<AutoConfigTokenContentV1>(&content)
            .map_err(|_| AutoConfigError::InvalidToken)?;

        let webhook =
            Webhook::new(&content.endpoint_secret).map_err(|_| AutoConfigError::InvalidToken)?;

        let svix = Svix::new(
            content.token_plaintext,
            Some(SvixOptions {
                server_url: Some(content.server_url),
                ..Default::default()
            }),
        );

        Ok(Self {
            endpoint,
            endpoint_id: content.endpoint_id,
            webhook,
            svix,
        })
    }

    pub async fn subscribe(&self) -> Result<EndpointOut> {
        InternalApi::new(self.svix.configuration_arc())
            .auto_config()
            .update(
                "-".to_string(),
                self.endpoint_id.clone(),
                SubscribeIn::new(self.endpoint.clone()),
            )
            .await
    }

    pub fn verify<HM: HeaderMap>(
        &self,
        payload: &[u8],
        headers: &HM,
    ) -> std::result::Result<(), WebhookError> {
        self.webhook.verify(payload, headers)
    }
}
