// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_headers_out::EndpointHeadersOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OtelTracingConfigOut {
    pub url: String,

    pub headers: EndpointHeadersOut,
}
