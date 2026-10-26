// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a PostHog sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostHogConfigIn {
    /// The PostHog ingestion domain.
    ///
    /// This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud,
    /// and your self-hosted domain for self-hosted instances. Note the `.i.` in
    /// the Cloud hosts (`us.posthog.com` and `eu.posthog.com` are
    /// different, private endpoints).
    pub domain: String,

    /// The PostHog project API key.
    ///
    /// Sent as the `api_key` field in the body of each `/batch/` request.
    #[serde(rename = "projectToken")]
    pub project_token: String,
}

impl PostHogConfigIn {
    pub fn new(domain: String, project_token: String) -> Self {
        Self {
            domain,
            project_token,
        }
    }
}
