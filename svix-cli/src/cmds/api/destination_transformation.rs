// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct DestinationTransformationArgs {
    #[command(subcommand)]
    pub command: DestinationTransformationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum DestinationTransformationCommands {
    /// Get the transformation code associated with this destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination transformation get app_abc000000000000000000000000 DESTINATION_ID\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
    Get {
        app_id: String,
        destination_id: String,
    },
    /// Set or unset the transformation code associated with this destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination transformation patch app_abc000000000000000000000000 DESTINATION_ID {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"code\": \"...\"
}\n\nExample response:
{
}\n")]
    Patch {
        app_id: String,
        destination_id: String,
        destination_transform_in: Option<crate::json::JsonOf<DestinationTransformIn>>,
    },
}

impl DestinationTransformationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Get {
                app_id,
                destination_id,
            } => {
                let resp = client
                    .destination()
                    .transformation()
                    .get(app_id, destination_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Patch {
                app_id,
                destination_id,
                destination_transform_in,
            } => {
                let resp = client
                    .destination()
                    .transformation()
                    .patch(
                        app_id,
                        destination_id,
                        destination_transform_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
