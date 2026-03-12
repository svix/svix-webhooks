// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    adobe_sign_config_out::AdobeSignConfigOut, airwallex_config_out::AirwallexConfigOut,
    checkbook_config_out::CheckbookConfigOut, cron_config::CronConfig,
    docusign_config_out::DocusignConfigOut, easypost_config_out::EasypostConfigOut,
    github_config_out::GithubConfigOut, hubspot_config_out::HubspotConfigOut,
    orum_io_config_out::OrumIoConfigOut, panda_doc_config_out::PandaDocConfigOut,
    port_io_config_out::PortIoConfigOut, rutter_config_out::RutterConfigOut,
    segment_config_out::SegmentConfigOut, shopify_config_out::ShopifyConfigOut,
    slack_config_out::SlackConfigOut, stripe_config_out::StripeConfigOut,
    svix_config_out::SvixConfigOut, telnyx_config_out::TelnyxConfigOut,
    vapi_config_out::VapiConfigOut, veriff_config_out::VeriffConfigOut,
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

    pub metadata: std::collections::HashMap<String, String>,

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
    #[serde(rename = "checkbook")]
    Checkbook(CheckbookConfigOut),
    #[serde(rename = "clerk")]
    Clerk(SvixConfigOut),
    #[serde(rename = "docusign")]
    Docusign(DocusignConfigOut),
    #[serde(rename = "easypost")]
    Easypost(EasypostConfigOut),
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
    #[serde(rename = "orum-io")]
    OrumIo(OrumIoConfigOut),
    #[serde(rename = "panda-doc")]
    PandaDoc(PandaDocConfigOut),
    #[serde(rename = "port-io")]
    PortIo(PortIoConfigOut),
    #[serde(rename = "pleo")]
    Pleo(SvixConfigOut),
    #[serde(rename = "replicate")]
    Replicate(SvixConfigOut),
    #[serde(rename = "resend")]
    Resend(SvixConfigOut),
    #[serde(rename = "rutter")]
    Rutter(RutterConfigOut),
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
    #[serde(rename = "telnyx")]
    Telnyx(TelnyxConfigOut),
    #[serde(rename = "vapi")]
    Vapi(VapiConfigOut),
    #[serde(rename = "open-ai")]
    OpenAi(SvixConfigOut),
    #[serde(rename = "render")]
    Render(SvixConfigOut),
    #[serde(rename = "veriff")]
    Veriff(VeriffConfigOut),
    #[serde(rename = "airwallex")]
    Airwallex(AirwallexConfigOut),
}

#[allow(clippy::derivable_impls)]
impl Default for IngestSourceOutConfig {
    fn default() -> Self {
        Self::GenericWebhook
    }
}
