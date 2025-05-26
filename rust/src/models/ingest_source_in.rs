// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    adobe_sign_config::AdobeSignConfig, cron_config::CronConfig, docusign_config::DocusignConfig,
    github_config::GithubConfig, hubspot_config::HubspotConfig, panda_doc_config::PandaDocConfig,
    segment_config::SegmentConfig, shopify_config::ShopifyConfig, slack_config::SlackConfig,
    stripe_config::StripeConfig, svix_config::SvixConfig, zoom_config::ZoomConfig,
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
    #[serde(rename = "generic-webhook")]
    GenericWebhook,
    #[serde(rename = "cron")]
    Cron(CronConfig),
    #[serde(rename = "adobe-sign")]
    AdobeSign(AdobeSignConfig),
    #[serde(rename = "beehiiv")]
    Beehiiv(SvixConfig),
    #[serde(rename = "brex")]
    Brex(SvixConfig),
    #[serde(rename = "clerk")]
    Clerk(SvixConfig),
    #[serde(rename = "docusign")]
    Docusign(DocusignConfig),
    #[serde(rename = "github")]
    Github(GithubConfig),
    #[serde(rename = "guesty")]
    Guesty(SvixConfig),
    #[serde(rename = "hubspot")]
    Hubspot(HubspotConfig),
    #[serde(rename = "incident-io")]
    IncidentIo(SvixConfig),
    #[serde(rename = "lithic")]
    Lithic(SvixConfig),
    #[serde(rename = "nash")]
    Nash(SvixConfig),
    #[serde(rename = "panda-doc")]
    PandaDoc(PandaDocConfig),
    #[serde(rename = "pleo")]
    Pleo(SvixConfig),
    #[serde(rename = "replicate")]
    Replicate(SvixConfig),
    #[serde(rename = "resend")]
    Resend(SvixConfig),
    #[serde(rename = "safebase")]
    Safebase(SvixConfig),
    #[serde(rename = "sardine")]
    Sardine(SvixConfig),
    #[serde(rename = "segment")]
    Segment(SegmentConfig),
    #[serde(rename = "shopify")]
    Shopify(ShopifyConfig),
    #[serde(rename = "slack")]
    Slack(SlackConfig),
    #[serde(rename = "stripe")]
    Stripe(StripeConfig),
    #[serde(rename = "stych")]
    Stych(SvixConfig),
    #[serde(rename = "svix")]
    Svix(SvixConfig),
    #[serde(rename = "zoom")]
    Zoom(ZoomConfig),
}

impl Default for IngestSourceInConfig {
    fn default() -> Self {
        Self::GenericWebhook
    }
}
