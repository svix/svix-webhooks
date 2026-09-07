use crate::{
    api::{Svix, SvixOptions},
    api_internal,
    error::Result,
    models::{EndpointIn, EndpointOut, SubscribeIn},
    webhooks::{Webhook, WebhookError},
};

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use http::HeaderMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoConfigTokenContentV1 {
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoConfigTokenContentV2 {
    #[serde(rename = "aid")]
    pub app_id: String,
    #[serde(rename = "sid")]
    pub autoconfig_id: String,
    #[serde(rename = "surl")]
    pub server_url: String,
    #[serde(rename = "esec")]
    pub endpoint_secret: String,
    #[serde(rename = "tok")]
    pub token_plaintext: String,
}

pub enum AutoConfigToken {
    V1(AutoConfigTokenContentV1),
    V2(AutoConfigTokenContentV2),
}

pub struct AutoConfig {
    app_id: String,
    endpoint_id: Option<String>,
    autoconfig_id: Option<String>,
    endpoint: EndpointIn,
    webhook: Webhook,
    svix: Svix,
}

const AUTOCONFIG_TOKEN_PREFIX_V1: &str = "auto_v1_";
const AUTOCONFIG_TOKEN_PREFIX_V2: &str = "auto_v2_";
const UNSUPPORTED_TOKEN_VERSION: &str =
    "Unsupported token version. You might need to update the Svix SDK to use this token";

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AutoConfigError {
    #[error("{}", .detail.unwrap_or("invalid token"))]
    InvalidToken { detail: Option<&'static str> },
}

fn parse_token_payload(token: &str, prefix: &str) -> std::result::Result<Vec<u8>, AutoConfigError> {
    let token = token
        .strip_prefix(prefix)
        .ok_or(AutoConfigError::InvalidToken {
            detail: Some(UNSUPPORTED_TOKEN_VERSION),
        })?;

    BASE64_STANDARD
        .decode(token)
        .map_err(|_| AutoConfigError::InvalidToken { detail: None })
}

fn decode_token<T: DeserializeOwned>(
    token: &str,
    prefix: &str,
) -> std::result::Result<T, AutoConfigError> {
    let decoded = parse_token_payload(token, prefix)?;
    serde_json::from_slice(&decoded).map_err(|_| AutoConfigError::InvalidToken { detail: None })
}

pub fn decode_autoconfig_token_v1(
    token: &str,
) -> std::result::Result<AutoConfigTokenContentV1, AutoConfigError> {
    decode_token(token, AUTOCONFIG_TOKEN_PREFIX_V1)
}

pub fn decode_autoconfig_token_v2(
    token: &str,
) -> std::result::Result<AutoConfigTokenContentV2, AutoConfigError> {
    decode_token(token, AUTOCONFIG_TOKEN_PREFIX_V2)
}

pub fn decode_autoconfig_token(
    token: &str,
) -> std::result::Result<AutoConfigToken, AutoConfigError> {
    if token.starts_with(AUTOCONFIG_TOKEN_PREFIX_V1) {
        Ok(AutoConfigToken::V1(decode_autoconfig_token_v1(token)?))
    } else if token.starts_with(AUTOCONFIG_TOKEN_PREFIX_V2) {
        Ok(AutoConfigToken::V2(decode_autoconfig_token_v2(token)?))
    } else {
        Err(AutoConfigError::InvalidToken {
            detail: Some(UNSUPPORTED_TOKEN_VERSION),
        })
    }
}

impl AutoConfig {
    pub fn new(token: String, endpoint: EndpointIn) -> std::result::Result<Self, AutoConfigError> {
        let decoded = decode_autoconfig_token(&token)?;

        let (app_id, server_url, endpoint_secret, token_plaintext, endpoint_id, autoconfig_id) =
            match decoded {
                AutoConfigToken::V1(content) => (
                    content.app_id,
                    content.server_url,
                    content.endpoint_secret,
                    content.token_plaintext,
                    Some(content.endpoint_id),
                    None,
                ),
                AutoConfigToken::V2(content) => (
                    content.app_id,
                    content.server_url,
                    content.endpoint_secret,
                    content.token_plaintext,
                    None,
                    Some(content.autoconfig_id),
                ),
            };

        let webhook = Webhook::new(&endpoint_secret)
            .map_err(|_| AutoConfigError::InvalidToken { detail: None })?;

        let svix = Svix::new(
            token_plaintext,
            Some(SvixOptions {
                server_url: Some(server_url),
                ..Default::default()
            }),
        );

        Ok(Self {
            app_id,
            endpoint_id,
            autoconfig_id,
            endpoint,
            webhook,
            svix,
        })
    }

    pub async fn subscribe(&self) -> Result<EndpointOut> {
        if let Some(autoconfig_id) = &self.autoconfig_id {
            return api_internal::endpoint_autoconfig(self.svix.cfg())
                .subscribe(
                    self.app_id.clone(),
                    autoconfig_id.clone(),
                    self.endpoint.clone(),
                )
                .await;
        }

        let mut subscribe_in = SubscribeIn::new();
        subscribe_in.endpoint = Some(self.endpoint.clone());

        api_internal::endpoint_auto_config_deprecated(self.svix.cfg())
            .update(
                self.app_id.clone(),
                self.endpoint_id.clone().expect("v1 tokens set endpoint_id"),
                subscribe_in,
            )
            .await
    }

    pub fn verify(
        &self,
        payload: &[u8],
        headers: &HeaderMap,
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
        let base64_json = BASE64_STANDARD.encode(json);
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V1}{base64_json}",);
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
        let base64_json = BASE64_STANDARD.encode(json);
        let token = format!("wrong_{base64_json}");
        assert!(matches!(
            decode_autoconfig_token_v1(&token),
            Err(AutoConfigError::InvalidToken {
                detail: Some(UNSUPPORTED_TOKEN_VERSION)
            })
        ));
    }

    #[test]
    fn decode_autoconfig_token_v1_rejects_invalid_json() {
        let base64_not_json = BASE64_STANDARD.encode("not json");
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V1}{base64_not_json}",);
        assert!(matches!(
            decode_autoconfig_token_v1(&token),
            Err(AutoConfigError::InvalidToken { detail: None })
        ));
    }

    #[test]
    fn decode_autoconfig_token_v2_parses_payload() {
        let json = r#"{"aid":"app_1","sid":"acfg_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"#;
        let base64_json = BASE64_STANDARD.encode(json);
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V2}{base64_json}",);
        let content = decode_autoconfig_token_v2(&token).expect("valid token");

        assert_eq!(content.app_id, "app_1");
        assert_eq!(content.autoconfig_id, "acfg_2");
        assert_eq!(content.server_url, "https://api.example.test");
        assert_eq!(content.endpoint_secret, "whsec_Zm9v");
        assert_eq!(content.token_plaintext, "sk_test_xyz");
    }

    #[test]
    fn decode_autoconfig_token_v2_rejects_bad_prefix() {
        let json = r#"{"aid":"a","sid":"s","surl":"https://x","esec":"whsec_Zm9v","tok":"t"}"#;
        let base64_json = BASE64_STANDARD.encode(json);
        let token = format!("wrong_{base64_json}");
        assert!(matches!(
            decode_autoconfig_token_v2(&token),
            Err(AutoConfigError::InvalidToken {
                detail: Some(UNSUPPORTED_TOKEN_VERSION)
            })
        ));
    }

    #[test]
    fn decode_autoconfig_token_v2_rejects_invalid_json() {
        let base64_not_json = BASE64_STANDARD.encode("not json");
        let token = format!("{AUTOCONFIG_TOKEN_PREFIX_V2}{base64_not_json}",);
        assert!(matches!(
            decode_autoconfig_token_v2(&token),
            Err(AutoConfigError::InvalidToken { detail: None })
        ));
    }

    #[test]
    fn decode_autoconfig_token_dispatches_and_rejects_unknown_prefix() {
        let v1_json = r#"{"aid":"app_1","eid":"ep_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"#;
        let v1_token = format!(
            "{AUTOCONFIG_TOKEN_PREFIX_V1}{}",
            BASE64_STANDARD.encode(v1_json)
        );
        assert!(matches!(
            decode_autoconfig_token(&v1_token),
            Ok(AutoConfigToken::V1(_))
        ));

        let v2_json = r#"{"aid":"app_1","sid":"acfg_2","surl":"https://api.example.test","esec":"whsec_Zm9v","tok":"sk_test_xyz"}"#;
        let v2_token = format!(
            "{AUTOCONFIG_TOKEN_PREFIX_V2}{}",
            BASE64_STANDARD.encode(v2_json)
        );
        assert!(matches!(
            decode_autoconfig_token(&v2_token),
            Ok(AutoConfigToken::V2(_))
        ));

        let unknown = format!("wrong_{}", BASE64_STANDARD.encode(v1_json));
        assert!(matches!(
            decode_autoconfig_token(&unknown),
            Err(AutoConfigError::InvalidToken {
                detail: Some(UNSUPPORTED_TOKEN_VERSION)
            })
        ));
    }
}
