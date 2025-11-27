// this file is @generated
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum AppPortalCapability {
    #[default]
    #[serde(rename = "ViewBase")]
    ViewBase,
    #[serde(rename = "ViewEndpointSecret")]
    ViewEndpointSecret,
    #[serde(rename = "ManageEndpointSecret")]
    ManageEndpointSecret,
    #[serde(rename = "ManageTransformations")]
    ManageTransformations,
    #[serde(rename = "CreateAttempts")]
    CreateAttempts,
    #[serde(rename = "ManageEndpoint")]
    ManageEndpoint,
}

impl fmt::Display for AppPortalCapability {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let value = match self {
            Self::ViewBase => "ViewBase",
            Self::ViewEndpointSecret => "ViewEndpointSecret",
            Self::ManageEndpointSecret => "ManageEndpointSecret",
            Self::ManageTransformations => "ManageTransformations",
            Self::CreateAttempts => "CreateAttempts",
            Self::ManageEndpoint => "ManageEndpoint",
        };
        f.write_str(value)
    }
}
