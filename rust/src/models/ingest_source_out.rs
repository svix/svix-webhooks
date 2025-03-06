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
    #[serde(rename = "generic-webhook")]
    GenericWebhook,
    #[serde(rename = "cron")]
    Cron(CronConfig),
    #[serde(rename = "adobe-sign")]
    AdobeSign(AdobeSignConfigOut),
    #[serde(rename = "beehiiv")]
    Beehiiv(SvixConfigOut),
    #[serde(rename = "brex")]
    Brex(SvixConfigOut),
    #[serde(rename = "clerk")]
    Clerk(SvixConfigOut),
    #[serde(rename = "docusign")]
    Docusign(DocusignConfigOut),
    #[serde(rename = "github")]
    Github(GithubConfigOut),
    #[serde(rename = "guesty")]
    Guesty(SvixConfigOut),
    #[serde(rename = "hubspot")]
    Hubspot(HubspotConfigOut),
    #[serde(rename = "incident-io")]
    IncidentIo(SvixConfigOut),
    #[serde(rename = "lithic")]
    Lithic(SvixConfigOut),
    #[serde(rename = "nash")]
    Nash(SvixConfigOut),
    #[serde(rename = "pleo")]
    Pleo(SvixConfigOut),
    #[serde(rename = "replicate")]
    Replicate(SvixConfigOut),
    #[serde(rename = "resend")]
    Resend(SvixConfigOut),
    #[serde(rename = "safebase")]
    Safebase(SvixConfigOut),
    #[serde(rename = "sardine")]
    Sardine(SvixConfigOut),
    #[serde(rename = "segment")]
    Segment(SegmentConfigOut),
    #[serde(rename = "shopify")]
    Shopify(ShopifyConfigOut),
    #[serde(rename = "slack")]
    Slack(SlackConfigOut),
    #[serde(rename = "stripe")]
    Stripe(StripeConfigOut),
    #[serde(rename = "stych")]
    Stych(SvixConfigOut),
    #[serde(rename = "svix")]
    Svix(SvixConfigOut),
    #[serde(rename = "zoom")]
    Zoom(ZoomConfigOut),
}

impl Default for IngestSourceOutConfig {
    fn default() -> Self {
        Self::GenericWebhook
    }
}
