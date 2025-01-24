// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TransformationTemplateKind {
    #[default]
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "CustomerIO")]
    CustomerIo,
    #[serde(rename = "Discord")]
    Discord,
    #[serde(rename = "Hubspot")]
    Hubspot,
    #[serde(rename = "Inngest")]
    Inngest,
    #[serde(rename = "Salesforce")]
    Salesforce,
    #[serde(rename = "Segment")]
    Segment,
    #[serde(rename = "Slack")]
    Slack,
    #[serde(rename = "Teams")]
    Teams,
    #[serde(rename = "TriggerDev")]
    TriggerDev,
    #[serde(rename = "Windmill")]
    Windmill,
    #[serde(rename = "Zapier")]
    Zapier,
}

impl fmt::Display for TransformationTemplateKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom => f.write_str("Custom"),
            Self::CustomerIo => f.write_str("CustomerIO"),
            Self::Discord => f.write_str("Discord"),
            Self::Hubspot => f.write_str("Hubspot"),
            Self::Inngest => f.write_str("Inngest"),
            Self::Salesforce => f.write_str("Salesforce"),
            Self::Segment => f.write_str("Segment"),
            Self::Slack => f.write_str("Slack"),
            Self::Teams => f.write_str("Teams"),
            Self::TriggerDev => f.write_str("TriggerDev"),
            Self::Windmill => f.write_str("Windmill"),
            Self::Zapier => f.write_str("Zapier"),
        }
    }
}
