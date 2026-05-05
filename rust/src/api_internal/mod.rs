pub mod endpoint_auto_config;

use crate::Configuration;
use endpoint_auto_config::EndpointAutoConfig;

pub(crate) fn endpoint_auto_config(cfg: &Configuration) -> EndpointAutoConfig<'_> {
    EndpointAutoConfig::new(cfg)
}
