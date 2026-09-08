// this file is @generated
use serde::{Deserialize, Serialize};

use super::status::Status;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AutoConfigSubscriptionOut {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "tokenCensored")]
    pub token_censored: String,

    /// The AutoConfigSubscription's ID.
    pub id: String,

    /// The Endpoint's ID.
    #[serde(rename = "endpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endp_id: Option<String>,

    /// The StreamSink's ID.
    #[serde(rename = "destId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_id: Option<String>,

    pub status: Status,
}
