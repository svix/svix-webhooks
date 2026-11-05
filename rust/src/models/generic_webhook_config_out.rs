// this file is @generated
use serde::{Deserialize, Serialize};

use super::generic_webhook_response::GenericWebhookResponse;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenericWebhookConfigOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<GenericWebhookResponse>,
}
