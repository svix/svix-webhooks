use crate::{
    api::{EndpointIn, EndpointOut, Svix, SvixOptions},
    api_internal,
    error::Result,
    models::SubscribeIn,
    webhooks::{HeaderMap, Webhook, WebhookError},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoConfigTokenContentV1 {
    #[serde(rename = "aid")]
    pub app_id: String,
    #[serde(rename = "eid")]
    pub endpoint_id: String,
    #[serde(rename = "surl")]
    pub server_url: String,
    #[serde(rename = "esec")]
    pub endpoint_secret: String,
    #[serde(rename = "tok")]
    pub token_plaintext: String,
}

pub struct AutoConfig {
    app_id: String,
    endpoint_id: String,
    endpoint: EndpointIn,
    webhook: Webhook,
    svix: Svix,
}

const AUTOCONFIG_TOKEN_PREFIX_V1: &str = "auto_v1_";

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AutoConfigError {
    #[error("invalid token")]
    InvalidToken,
}

fn decode_autoconfig_token_v1(
    token: &str,
) -> std::result::Result<AutoConfigTokenContentV1, AutoConfigError> {
    let token = token
        .strip_prefix(AUTOCONFIG_TOKEN_PREFIX_V1)
        .ok_or(AutoConfigError::InvalidToken)?;

    let decoded = base64::decode(token).map_err(|_| AutoConfigError::InvalidToken)?;

    serde_json::from_slice::<AutoConfigTokenContentV1>(&decoded)
        .map_err(|_| AutoConfigError::InvalidToken)
}

impl AutoConfig {
    pub fn new(token: String, endpoint: EndpointIn) -> std::result::Result<Self, AutoConfigError> {
        let content = decode_autoconfig_token_v1(&token)?;
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
            app_id: content.app_id,
            endpoint_id: content.endpoint_id,
            endpoint,
            webhook,
            svix,
        })
    }

    pub async fn subscribe(&self) -> Result<EndpointOut> {
        api_internal::endpoint_auto_config(self.svix.cfg())
            .update(
                self.app_id.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_autoconfig_token_v1_parses_payload() {
        let json = r#"{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"#;
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V1}{}", base64::encode(json));
        let content = decode_autoconfig_token_v1(&token).expect("valid token");

        assert_eq!(content.app_id, "app_1");
        assert_eq!(content.endpoint_id, "ep_2");
        assert_eq!(content.server_url, "https://api.example.test");
        assert_eq!(content.endpoint_secret, "whsec_Zm9v");
        assert_eq!(content.token_plaintext, "sk_test_xyz");
    }

    #[test]
    fn decode_autoconfig_token_v1_rejects_bad_prefix() {
        let json = r#"{"aid":"a","eid":"e","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}"#;
        let token = format!("wrong_{}", base64::encode(json));
        assert!(matches!(
            decode_autoconfig_token_v1(&token),
            Err(AutoConfigError::InvalidToken)
        ));
    }

    #[test]
    fn decode_autoconfig_token_v1_rejects_invalid_json() {
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V1}{}", base64::encode("not json"));
        assert!(matches!(
            decode_autoconfig_token_v1(&token),
            Err(AutoConfigError::InvalidToken)
        ));
    }
}
