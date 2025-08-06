// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

use super::operational_webhook_endpoint::OperationalWebhookEndpointArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct OperationalWebhookArgs {
    #[command(subcommand)]
    pub command: OperationalWebhookCommands,
}

#[derive(Subcommand)]
pub enum OperationalWebhookCommands {
    Endpoint(OperationalWebhookEndpointArgs),
}

impl OperationalWebhookCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Endpoint(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}
