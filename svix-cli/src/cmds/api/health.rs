// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct HealthArgs {
    #[command(subcommand)]
    pub command: HealthCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum HealthCommands {
    /// Verify the API server is up and running.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix health get\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Get {},
}

impl HealthCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Get {} => {
                client.health().get().await?;
            }
        }

        Ok(())
    }
}
