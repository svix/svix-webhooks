#[allow(dead_code)]
pub mod autoconfig;
#[allow(dead_code)]
pub mod autoconfig_destination;
#[allow(dead_code)]
pub mod autoconfig_endpoint;
pub mod endpoint_auto_config_deprecated;
#[allow(dead_code)]
mod endpoint;
#[allow(dead_code)]
mod message;
pub mod message_pollerv2;

use crate::Configuration;
use autoconfig::Autoconfig;
pub(crate) use autoconfig_destination::AutoconfigDestination;
pub(crate) use autoconfig_endpoint::AutoconfigEndpoint;
use endpoint_auto_config_deprecated::EndpointAutoConfigDeprecated;
use message_pollerv2::MessagePollerv2;

pub(crate) fn autoconfig(cfg: &Configuration) -> Autoconfig<'_> {
    Autoconfig::new(cfg)
}

pub(crate) fn endpoint_auto_config_deprecated(
    cfg: &Configuration,
) -> EndpointAutoConfigDeprecated<'_> {
    EndpointAutoConfigDeprecated::new(cfg)
}

pub(crate) fn message_pollerv2(cfg: &Configuration) -> MessagePollerv2<'_> {
    MessagePollerv2::new(cfg)
}
