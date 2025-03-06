// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    adobe_sign_config::AdobeSignConfig, cron_config::CronConfig, docusign_config::DocusignConfig,
    github_config::GithubConfig, hubspot_config::HubspotConfig, segment_config::SegmentConfig,
    shopify_config::ShopifyConfig, slack_config::SlackConfig, stripe_config::StripeConfig,
    svix_config::SvixConfig, zoom_config::ZoomConfig,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestSourceIn {
    pub name: String,

    /// The Source's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(flatten)]
    pub config: IngestSourceInConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum IngestSourceInConfig {
    GenericWebhook,
    Cron(CronConfig),
    AdobeSign(AdobeSignConfig),
    Beehiiv(SvixConfig),
    Brex(SvixConfig),
    Clerk(SvixConfig),
    Docusign(DocusignConfig),
    Github(GithubConfig),
    Guesty(SvixConfig),
    Hubspot(HubspotConfig),
    IncidentIo(SvixConfig),
    Lithic(SvixConfig),
    Nash(SvixConfig),
    Pleo(SvixConfig),
    Replicate(SvixConfig),
    Resend(SvixConfig),
    Safebase(SvixConfig),
    Sardine(SvixConfig),
    Segment(SegmentConfig),
    Shopify(ShopifyConfig),
    Slack(SlackConfig),
    Stripe(StripeConfig),
    Stych(SvixConfig),
    Svix(SvixConfig),
    Zoom(ZoomConfig),
}

impl Default for IngestSourceInConfig {
    fn default() -> Self {
        Self::GenericWebhook
    }
}
