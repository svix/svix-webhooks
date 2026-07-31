// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestEndpointTransformationArgs {
    #[command(subcommand)]
    pub command: IngestEndpointTransformationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum IngestEndpointTransformationCommands {
    /// Get the transformation code associated with this ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint transformation transformation src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
    Transformation {
        source_id: String,
        endpoint_id: String,
    },
    /// Set or unset the transformation code associated with this ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint transformation patch src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
    Patch {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_transformation_patch:
            Option<crate::json::JsonOf<IngestEndpointTransformationPatch>>,
    },
}

impl IngestEndpointTransformationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Transformation {
                source_id,
                endpoint_id,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .transformation()
                    .transformation(source_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Patch {
                source_id,
                endpoint_id,
                ingest_endpoint_transformation_patch,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .transformation()
                    .patch(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_transformation_patch
                            .unwrap_or_default()
                            .into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
