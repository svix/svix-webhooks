#[allow(dead_code)]
mod destination;
#[allow(dead_code)]
pub mod destination_autoconfig;
#[allow(dead_code)]
mod endpoint;
pub mod endpoint_auto_config_deprecated;
#[allow(dead_code)]
pub mod endpoint_autoconfig;
#[allow(dead_code)]
mod message;
pub mod message_pollerv2;

use crate::Configuration;
use destination_autoconfig::DestinationAutoconfig;
use endpoint_auto_config_deprecated::EndpointAutoConfigDeprecated;
use endpoint_autoconfig::EndpointAutoconfig;
use message_pollerv2::MessagePollerv2;

#[allow(dead_code)]
pub(crate) fn destination_autoconfig(cfg: &Configuration) -> DestinationAutoconfig<'_> {
    DestinationAutoconfig::new(cfg)
}

pub(crate) fn endpoint_auto_config_deprecated(
    cfg: &Configuration,
) -> EndpointAutoConfigDeprecated<'_> {
    EndpointAutoConfigDeprecated::new(cfg)
}

#[allow(dead_code)]
pub(crate) fn endpoint_autoconfig(cfg: &Configuration) -> EndpointAutoconfig<'_> {
    EndpointAutoconfig::new(cfg)
}

pub(crate) fn message_pollerv2(cfg: &Configuration) -> MessagePollerv2<'_> {
    MessagePollerv2::new(cfg)
}
