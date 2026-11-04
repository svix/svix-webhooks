pub mod autoconfig_subscription;
pub mod autoconfig_subscription_destination;
pub mod autoconfig_subscription_endpoint;
pub mod endpoint_auto_config_deprecated;
pub mod message_pollerv2;

use crate::Configuration;
use autoconfig_subscription::AutoconfigSubscription;
use autoconfig_subscription_destination::AutoconfigSubscriptionDestination;
use autoconfig_subscription_endpoint::AutoconfigSubscriptionEndpoint;
use endpoint_auto_config_deprecated::EndpointAutoConfigDeprecated;
use message_pollerv2::MessagePollerv2;

pub(crate) fn autoconfig_subscription(cfg: &Configuration) -> AutoconfigSubscription<'_> {
    AutoconfigSubscription::new(cfg)
}

pub(crate) fn endpoint_auto_config_deprecated(
    cfg: &Configuration,
) -> EndpointAutoConfigDeprecated<'_> {
    EndpointAutoConfigDeprecated::new(cfg)
}

pub(crate) fn message_pollerv2(cfg: &Configuration) -> MessagePollerv2<'_> {
    MessagePollerv2::new(cfg)
}
