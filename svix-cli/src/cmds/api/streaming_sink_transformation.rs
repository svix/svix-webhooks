// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingSinkTransformationArgs {
    #[command(subcommand)]
    pub command: StreamingSinkTransformationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum StreamingSinkTransformationCommands {
    /// Get the transformation code associated with this sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink transformation get strm_abc000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
    Get { stream_id: String, sink_id: String },
    /// Set or unset the transformation code associated with this sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink transformation patch strm_abc000000000000000000 sink_abc000000000000000000 {...}\n",
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
        stream_id: String,
        sink_id: String,
        sink_transform_in: Option<crate::json::JsonOf<SinkTransformIn>>,
    },
}

impl StreamingSinkTransformationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Get { stream_id, sink_id } => {
                let resp = client
                    .streaming()
                    .sink()
                    .transformation()
                    .get(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Patch {
                stream_id,
                sink_id,
                sink_transform_in,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .transformation()
                    .patch(
                        stream_id,
                        sink_id,
                        sink_transform_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
