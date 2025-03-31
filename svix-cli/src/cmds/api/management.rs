use clap::{Args, Subcommand};
use svix::api::*;

use super::management_authentication::ManagementAuthenticationArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ManagementArgs {
    #[command(subcommand)]
    pub command: ManagementCommands,
}

#[derive(Subcommand)]
pub enum ManagementCommands {
    Authentication(ManagementAuthenticationArgs),
}

impl ManagementCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Authentication(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}
