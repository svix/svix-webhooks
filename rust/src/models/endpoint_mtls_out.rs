// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_mtls_config_out::EndpointMtlsConfigOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointMtlsOut {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<EndpointMtlsConfigOut>,
}
