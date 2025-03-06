// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    adobe_sign_config_out::AdobeSignConfigOut, cron_config::CronConfig,
    docusign_config_out::DocusignConfigOut, github_config_out::GithubConfigOut,
    hubspot_config_out::HubspotConfigOut, segment_config_out::SegmentConfigOut,
    shopify_config_out::ShopifyConfigOut, slack_config_out::SlackConfigOut,
    stripe_config_out::StripeConfigOut, svix_config_out::SvixConfigOut,
    zoom_config_out::ZoomConfigOut,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestSourceOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The Source's ID.
    pub id: String,

    #[serde(rename = "ingestUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_url: Option<String>,

    pub name: String,

    /// The Source's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(flatten)]
    pub config: IngestSourceOutConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum IngestSourceOutConfig {
    GenericWebhook,
    Cron(CronConfig),
    AdobeSign(AdobeSignConfigOut),
    Beehiiv(SvixConfigOut),
    Brex(SvixConfigOut),
    Clerk(SvixConfigOut),
    Docusign(DocusignConfigOut),
    Github(GithubConfigOut),
    Guesty(SvixConfigOut),
    Hubspot(HubspotConfigOut),
    IncidentIo(SvixConfigOut),
    Lithic(SvixConfigOut),
    Nash(SvixConfigOut),
    Pleo(SvixConfigOut),
    Replicate(SvixConfigOut),
    Resend(SvixConfigOut),
    Safebase(SvixConfigOut),
    Sardine(SvixConfigOut),
    Segment(SegmentConfigOut),
    Shopify(ShopifyConfigOut),
    Slack(SlackConfigOut),
    Stripe(StripeConfigOut),
    Stych(SvixConfigOut),
    Svix(SvixConfigOut),
    Zoom(ZoomConfigOut),
}

impl Default for IngestSourceOutConfig {
    fn default() -> Self {
        Self::GenericWebhook
    }
}
