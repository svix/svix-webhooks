// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_oauth_config_out::EndpointOauthConfigOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointOauthOut {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<EndpointOauthConfigOut>,
}
