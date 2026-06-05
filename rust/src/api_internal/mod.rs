#[allow(dead_code)]
mod endpoint;
pub mod endpoint_auto_config;
#[allow(dead_code)]
mod message;
pub mod message_pollerv2;

use crate::Configuration;
use endpoint_auto_config::EndpointAutoConfig;
use message_pollerv2::MessagePollerv2;

pub(crate) fn endpoint_auto_config(cfg: &Configuration) -> EndpointAutoConfig<'_> {
    EndpointAutoConfig::new(cfg)
}

pub(crate) fn message_pollerv2(cfg: &Configuration) -> MessagePollerv2<'_> {
    MessagePollerv2::new(cfg)
}
