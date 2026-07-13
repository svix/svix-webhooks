// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EndpointTransformationArgs {
    #[command(subcommand)]
    pub command: EndpointTransformationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum EndpointTransformationCommands {
    /// Get the transformation code associated with this endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix endpoint transformation get app_abc000000000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"code\": \"...\",
  \"enabled\": true,
  \"variables\": {\"key\": \"...\"},
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Get { app_id: String, endpoint_id: String },
    /// Set or unset the transformation code associated with this endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix endpoint transformation patch app_abc000000000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"code\": \"function handler(webhook) { /* ... */ }\",
  \"enabled\": true,
  \"variables\": {\"key\": \"...\"}
}\n")]
    Patch {
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_patch: Option<crate::json::JsonOf<EndpointTransformationPatch>>,
    },
}

impl EndpointTransformationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Get {
                app_id,
                endpoint_id,
            } => {
                let resp = client
                    .endpoint()
                    .transformation()
                    .get(app_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Patch {
                app_id,
                endpoint_id,
                endpoint_transformation_patch,
            } => {
                client
                    .endpoint()
                    .transformation()
                    .patch(
                        app_id,
                        endpoint_id,
                        endpoint_transformation_patch
                            .unwrap_or_default()
                            .into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
